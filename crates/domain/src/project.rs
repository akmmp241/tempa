#[derive(Debug, Clone)]
pub struct Project {
    pub id: uuid::Uuid,
    pub host_id: uuid::Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
