use async_trait::async_trait;

#[async_trait]
pub trait HostRepository: Send + Sync {
    async fn insert(&self, host: domain::host::Host) -> anyhow::Result<()>;
    async fn get_all(&self) -> anyhow::Result<Vec<domain::host::Host>>;
    async fn get_by_id(&self, id: &str) -> anyhow::Result<domain::host::Host>;
    async fn update(&self, host: domain::host::Host) -> anyhow::Result<()>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
}