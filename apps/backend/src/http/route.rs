use crate::bootstrap::AppState;
use crate::http::host::handler::{
    create_host, delete_host, get_host_by_id, get_host_projects, get_hosts, observe_host_status,
    update_host_metadata,
};
use axum::response::Html;
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use http::StatusCode;
use serde_json::json;

const OPENAPI_YML: &str = include_str!("../../../../openapi.yml");

const SCALAR_UI_HTML: &str = r#"
<!doctype html>
<html>
  <head>
    <title>LogiTrack API Reference</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style>
      body { margin: 0; }
    </style>
  </head>
  <body>
    <script id="api-reference" data-url="/openapi.yml"></script>
    <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
  </body>
</html>
"#;

pub fn create_routes(state: AppState) -> Router {
    let app: Router<AppState> = Router::new().nest("/hosts", host_routes());

    Router::new()
        .merge(system_routes())
        .nest("/api/v1", app)
        .with_state(state)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not Found".to_owned()) })
}

fn system_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/openapi.yml",
            get(|| async { ([("content-type", "text/yml")], OPENAPI_YML) }),
        )
        .route("/docs", get(|| async { Html(SCALAR_UI_HTML) }))
        .route("/health", get(|| async { Json(json!({ "status": "ok" })) }))
}

fn host_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_hosts))
        .route("/", post(create_host))
        .route("/{host_id}", get(get_host_by_id))
        .route("/{host_id}", patch(update_host_metadata))
        .route("/{host_id}", delete(delete_host))
        .route("/{host_id}/status", get(observe_host_status))
        .route("/{host_id}/projects", get(get_host_projects))
}
