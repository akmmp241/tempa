use async_trait::async_trait;
use domain::project::Project;
use sqlx::types::Uuid;

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn get_by_host(
        &self,
        host_id: &Uuid,
        after: Option<Uuid>,
        limit: i64,
    ) -> anyhow::Result<Vec<Project>>;
}
