// pub mod bootstrap;

pub mod bootstrap;
pub mod http;

use crate::bootstrap::Bootstrap;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    fs::create_dir_all("logs").await?;
    log4rs::init_file("log4rs.yaml", Default::default())?;

    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await?;

    sqlx::migrate!("../../migrations").run(&pool).await?;

    let bootstrap = Bootstrap::new();

    bootstrap.run().await;

    Ok(())
}
