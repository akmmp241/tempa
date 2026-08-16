use anyhow::{Context, Result};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub docker: DockerConfig,
    pub deployment: DeploymentConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct DockerConfig {
    pub socket: String,
}

#[derive(Debug, Clone)]
pub struct DeploymentConfig {
    pub workspace_dir: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            server: ServerConfig {
                host: env::var("TEMPA_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
                port: env::var("TEMPA_PORT")
                    .unwrap_or_else(|_| "3000".into())
                    .parse()
                    .context("invalid TEMPA_PORT")?,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").context("DATABASE_URL is required")?,
            },
            docker: DockerConfig {
                socket: env::var("DOCKER_HOST")
                    .unwrap_or_else(|_| "unix:///var/run/docker.sock".into()),
            },
            deployment: DeploymentConfig {
                workspace_dir: env::var("TEMPA_WORKSPACE_DIR")
                    .unwrap_or_else(|_| "./data/workspaces".into()),
            },
        })
    }
}

pub async fn db_pool(db_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .unwrap()
}

pub async fn init_logger() -> Result<()> {
    fs::create_dir_all("logs").await?;
    log4rs::init_file("log4rs.yaml", Default::default())?;
    Ok(())
}
