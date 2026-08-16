use crate::ports::host_repository::HostRepository;
use async_trait::async_trait;
use domain::host::{Host, HostStatus, HostType};
use sqlx::types::{
    Uuid,
    chrono::{DateTime, Utc},
};
use sqlx::{AssertSqlSafe, FromRow, PgPool};

pub struct PostgresHostRepository {
    pool: PgPool,
}

#[derive(FromRow)]
struct HostRow {
    id: Uuid,
    name: String,
    #[sqlx(rename = "type")]
    host_type: HostType,
    docker_endpoint: String,
    status: HostStatus,
    last_seen_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl HostRow {
    fn into_host(self) -> Host {
        Host {
            id: self.id,
            name: self.name,
            _type: self.host_type,
            docker_endpoint: self.docker_endpoint,
            status: self.status,
            last_seen_at: self.last_seen_at.map(|value| value.naive_utc()),
            created_at: self.created_at.naive_utc(),
        }
    }
}

impl PostgresHostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HostRepository for PostgresHostRepository {
    async fn insert(&self, host: &Host) -> anyhow::Result<()> {
        let created_at = DateTime::<Utc>::from_naive_utc_and_offset(host.created_at, Utc);
        let last_seen_at = host
            .last_seen_at
            .map(|value| DateTime::<Utc>::from_naive_utc_and_offset(value, Utc));

        sqlx::query(
            "INSERT INTO hosts (id, name, type, docker_endpoint, status, last_seen_at, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
            .bind(&host.id)
            .bind(&host.name)
            .bind(&host._type.to_string())
            .bind(&host.docker_endpoint)
            .bind(&host.status.to_string())
            .bind(last_seen_at)
            .bind(created_at)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                log::error!("error inserting host: {}", e);
                e
            })?;

        Ok(())
    }

    async fn get_all(&self) -> anyhow::Result<Vec<Host>> {
        let rows = sqlx::query_as::<_, HostRow>(
            "SELECT id, name, type AS host_type, docker_endpoint, status, last_seen_at, created_at
             FROM hosts",
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(HostRow::into_host).collect())
    }

    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<Host>> {
        let id = Uuid::parse_str(id)?;
        let row = sqlx::query_as::<_, HostRow>(
            "SELECT id, name, type AS host_type, docker_endpoint, status, last_seen_at, created_at
             FROM hosts
             WHERE id = $1",
        )
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(Some(row.into_host()))
    }

    async fn get_by(&self, column: &str, value: &str) -> anyhow::Result<Option<Host>> {
        let column = match column {
            "name" => "name",
            "status" => "status",
            "type" => "type",
            _ => anyhow::bail!("invalid column"),
        };

        let query = format!(
            "SELECT id, name, type AS host_type, docker_endpoint, status, last_seen_at, created_at
            FROM hosts
            WHERE {} = $1",
            column
        );

        let row = sqlx::query_as::<_, HostRow>(AssertSqlSafe(query))
            .bind(value)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                log::error!("error fetching host by {}: {}", column, e);
                e
            })?;

        let host = match row {
            None => {
                log::debug!("host with {}: {} not found", column, value);
                return Ok(None);
            }
            Some(row) => row.into_host(),
        };

        Ok(Some(host))
    }

    async fn update(&self, host: Host) -> anyhow::Result<()> {
        let last_seen_at = host
            .last_seen_at
            .map(|value| DateTime::<Utc>::from_naive_utc_and_offset(value, Utc));

        sqlx::query(
            "UPDATE hosts
             SET name = $1, type = $2, docker_endpoint = $3, status = $4, last_seen_at = $5
             WHERE id = $6",
        )
            .bind(host.name)
            .bind(host._type)
            .bind(host.docker_endpoint)
            .bind(host.status)
            .bind(last_seen_at)
            .bind(host.id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        let id = Uuid::parse_str(id)?;

        sqlx::query("DELETE FROM hosts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
