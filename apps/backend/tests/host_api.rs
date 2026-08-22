use axum_test::TestServer;
use backend::application::host::HostService;
use backend::bootstrap::AppState;
use backend::http::route::create_routes;
use backend::infra::postgres::host_repository::PostgresHostRepository;
use backend::infra::postgres::project_repository::PostgresProjectRepository;
use backend::ports::host_repository::HostRepository;
use backend::ports::project_repository::ProjectRepository;
use serde_json::{Value, json};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;
use std::env;
use std::sync::Arc;

async fn test_server() -> (TestServer, PgPool) {
    let database_url = env::var("DATABASE_URL_TEST")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL_TEST or DATABASE_URL must be set for host_api tests");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("test database should be reachable");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("test database migrations should succeed");

    let host_repository: Arc<dyn HostRepository> =
        Arc::new(PostgresHostRepository::new(pool.clone()));
    let project_repository: Arc<dyn ProjectRepository> =
        Arc::new(PostgresProjectRepository::new(pool.clone()));
    let host_service = HostService::new(host_repository, project_repository);
    let state = AppState {
        host_service: Arc::new(host_service),
    };

    (TestServer::new(create_routes(state)), pool)
}

async fn cleanup_host(pool: &PgPool, name: &str) {
    sqlx::query("DELETE FROM hosts WHERE name = $1")
        .bind(name)
        .execute(pool)
        .await
        .expect("test host cleanup should succeed");
}

fn create_payload(name: &str) -> Value {
    json!({
        "name": name,
        "type": "local",
        "docker_endpoint": "unix:///var/run/docker.sock"
    })
}

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let (server, pool) = test_server().await;

    server
        .get("/health")
        .await
        .assert_status_ok()
        .assert_json(&json!({"status": "ok"}));

    pool.close().await;
}

#[tokio::test]
async fn create_and_get_host_round_trip() {
    let (server, pool) = test_server().await;
    let name = format!("test-host-{}", Uuid::new_v4());

    let created = server
        .post("/api/v1/hosts")
        .json(&create_payload(&name))
        .await;
    created.assert_status(http::StatusCode::CREATED);
    let created_body: Value = created.json();
    assert_eq!(created_body["success"], true);
    assert_eq!(created_body["data"]["name"], name);
    let host_id = created_body["data"]["id"]
        .as_str()
        .expect("created response should contain host id")
        .to_owned();

    let fetched = server.get(&format!("/api/v1/hosts/{host_id}")).await;
    fetched.assert_status_ok();
    let fetched_body: Value = fetched.json();
    assert_eq!(fetched_body["data"]["id"], host_id);
    assert_eq!(fetched_body["data"]["name"], name);

    let listed = server.get(&format!("/api/v1/hosts?q={name}&limit=1")).await;
    listed.assert_status_ok();
    let listed_body: Value = listed.json();
    assert!(
        listed_body["data"]
            .as_array()
            .unwrap()
            .iter()
            .any(|host| { host["id"] == host_id && host["name"] == name })
    );
    assert!(listed_body["meta"]["has_more"].is_boolean());

    cleanup_host(&pool, &name).await;
    pool.close().await;
}

#[tokio::test]
async fn invalid_create_payload_returns_unprocessable_entity() {
    let (server, pool) = test_server().await;

    server
        .post("/api/v1/hosts")
        .json(&json!({
            "name": "x",
            "type": "invalid",
            "docker_endpoint": "endpoint"
        }))
        .await
        .assert_status_unprocessable_entity();

    pool.close().await;
}

#[tokio::test]
async fn duplicate_host_name_returns_bad_request() {
    let (server, pool) = test_server().await;
    let name = format!("test-host-{}", Uuid::new_v4());

    server
        .post("/api/v1/hosts")
        .json(&create_payload(&name))
        .await
        .assert_status(http::StatusCode::CREATED);

    server
        .post("/api/v1/hosts")
        .json(&create_payload(&name))
        .await
        .assert_status_bad_request();

    cleanup_host(&pool, &name).await;
    pool.close().await;
}

#[tokio::test]
async fn update_delete_and_not_found_flow() {
    let (server, pool) = test_server().await;
    let name = format!("test-host-{}", Uuid::new_v4());
    let updated_name = format!("updated-host-{}", Uuid::new_v4());

    let created = server
        .post("/api/v1/hosts")
        .json(&create_payload(&name))
        .await;
    let created_body: Value = created.json();
    let host_id = created_body["data"]["id"]
        .as_str()
        .expect("created response should contain host id")
        .to_owned();

    let updated = server
        .patch(&format!("/api/v1/hosts/{host_id}"))
        .json(&json!({"name": updated_name}))
        .await;
    updated.assert_status_ok();
    assert_eq!(
        updated.json::<Value>()["data"]["data"]["name"],
        updated_name
    );

    server
        .delete(&format!("/api/v1/hosts/{host_id}"))
        .await
        .assert_status_no_content();

    server
        .get(&format!("/api/v1/hosts/{host_id}"))
        .await
        .assert_status_not_found();

    cleanup_host(&pool, &name).await;
    cleanup_host(&pool, &updated_name).await;
    pool.close().await;
}
