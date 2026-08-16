use crate::application::host::HostService;
use crate::config;
use crate::config::Config;
use crate::http::route;
use crate::infra::postgres::host_repository::PostgresHostRepository;
use crate::ports::host_repository::HostRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub host_service: Arc<HostService>,
}

pub struct Bootstrap {
    pub host: String,
    pub port: u16,
    pub state: AppState,
}

impl Bootstrap {
    pub async fn new(config: Config) -> Self {
        let db = config::db_pool(&config.database.url).await;

        let host_repository: Arc<dyn HostRepository> =
            Arc::new(PostgresHostRepository::new(db.clone()));

        let host_service = HostService::new(host_repository);

        let state = AppState {
            host_service: Arc::new(host_service),
        };

        Self {
            host: config.server.host,
            port: config.server.port,
            state,
        }
    }

    fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub async fn run(&self) {
        let app = route::create_routes(self.state.clone());

        log::info!("Starting HTTP server on {}", self.get_addr());

        let listener = tokio::net::TcpListener::bind(self.get_addr())
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
