use crate::http::route;

pub struct Bootstrap {
    pub host: String,
    pub port: u16,
}

impl Bootstrap {
    pub fn new() -> Self {
        Bootstrap { host: "0.0.0.0".to_string(), port: 8000 }
    }

    fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub async fn run(&self) {
        let app = route::create_routes();

        log::info!("Starting HTTP server on {}", self.get_addr());

        let listener = tokio::net::TcpListener::bind(self.get_addr()).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}