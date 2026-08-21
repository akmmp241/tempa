use serde::Serialize;
use sqlx::types::Uuid;

#[derive(Serialize, Debug)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub host_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub application_count: u16,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<domain::project::Project> for ProjectResponse {
    fn from(project: domain::project::Project) -> Self {
        Self {
            id: project.id,
            host_id: project.host_id,
            name: project.name,
            description: project.description,
            application_count: 0,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}
