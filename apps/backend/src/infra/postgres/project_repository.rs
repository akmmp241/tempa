use crate::ports::project_repository::ProjectRepository;
use async_trait::async_trait;
use domain::project::Project;
use sqlx::types::{
    Uuid,
    chrono::{DateTime, Utc},
};
use sqlx::{FromRow, PgPool};

pub struct PostgresProjectRepository {
    pool: PgPool,
}

#[derive(FromRow)]
struct ProjectRow {
    id: Uuid,
    host_id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ProjectRow {
    fn into_project(self) -> Project {
        Project {
            id: self.id,
            host_id: self.host_id,
            name: self.name,
            slug: self.slug,
            description: self.description,
            created_at: self.created_at.naive_utc(),
            updated_at: self.updated_at.naive_utc(),
        }
    }
}

impl PostgresProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectRepository for PostgresProjectRepository {
    async fn get_by_host(
        &self,
        host_id: &Uuid,
        after: Option<Uuid>,
        limit: i64,
    ) -> anyhow::Result<Vec<Project>> {
        let rows = match after {
            Some(after) => {
                sqlx::query_as::<_, ProjectRow>(
                    "SELECT id, host_id, name, slug, description, created_at, updated_at
                     FROM projects
                     WHERE host_id = $1 AND id > $2
                     ORDER BY id ASC
                     LIMIT $3",
                )
                .bind(host_id)
                .bind(after)
                .bind(limit)
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as::<_, ProjectRow>(
                    "SELECT id, host_id, name, slug, description, created_at, updated_at
                     FROM projects
                     WHERE host_id = $1
                     ORDER BY id ASC
                     LIMIT $2",
                )
                .bind(host_id)
                .bind(limit)
                .fetch_all(&self.pool)
                .await?
            }
        };

        Ok(rows.into_iter().map(ProjectRow::into_project).collect())
    }
}
