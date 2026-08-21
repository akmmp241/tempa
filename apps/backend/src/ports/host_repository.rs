use crate::ports::lib::PageRequest;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct HostPosition {
    pub created_at: DateTime<Utc>,
    pub id: Uuid,
}

#[async_trait]
pub trait HostRepository: Send + Sync {
    async fn insert(&self, host: &domain::host::Host) -> anyhow::Result<()>;
    async fn get_all(
        &self,
        query: PageRequest<HostPosition>,
    ) -> anyhow::Result<Vec<domain::host::Host>>;
    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<domain::host::Host>>;
    async fn get_by(&self, column: &str, value: &str)
    -> anyhow::Result<Option<domain::host::Host>>;
    async fn update(&self, host: &domain::host::Host) -> anyhow::Result<()>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
}
