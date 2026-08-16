pub mod application;
pub mod bootstrap;
pub mod config;
pub mod http;
pub mod infra;
pub mod ports;

use crate::bootstrap::Bootstrap;
use dotenvy::dotenv;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env()?;
    let db_pool = config::db_pool(&config.database.url).await;
    config::init_logger().await?;

    sqlx::migrate!("../../migrations").run(&db_pool).await?;

    Bootstrap::new(config).await.run().await;

    Ok(())
}
