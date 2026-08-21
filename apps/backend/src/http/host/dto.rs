use crate::http::project::dto::ProjectResponse;
use domain::host::{HostStatus, HostType};
use serde::{Deserialize, Serialize};
use sqlx::types::{Uuid, chrono};
use validator::{Validate, ValidationError};

fn validate_host_type(value: &str) -> Result<(), ValidationError> {
    let valid_types = ["local", "remote"];
    if valid_types.contains(&value) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_host_type"))
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct CreateHostRequest {
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    #[validate(
        length(max = 20),
        custom(function = "validate_host_type", message = "invalid host type")
    )]
    #[serde(rename = "type")]
    pub _type: String,
    pub docker_endpoint: String,
}

impl From<CreateHostRequest> for domain::host::Host {
    fn from(req: CreateHostRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: req.name,
            _type: req._type.into(),
            docker_endpoint: req.docker_endpoint,
            status: HostStatus::Unknown,
            last_seen_at: None,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct HostResponse {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: HostType,
    pub docker_endpoint: String,
    pub status: HostStatus,
    pub project_count: u16,
    pub last_seen_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
}

impl Into<HostResponse> for domain::host::Host {
    fn into(self) -> HostResponse {
        HostResponse {
            id: self.id,
            name: self.name,
            _type: self._type,
            docker_endpoint: self.docker_endpoint,
            status: self.status,
            project_count: 0,
            last_seen_at: self.last_seen_at,
            created_at: self.created_at,
        }
    }
}

pub type CreateHostResponse = HostResponse;

#[derive(Deserialize, Debug)]
pub struct GetAllHostRequest {
    pub cursor: Option<String>,
    pub limit: Option<i16>,
    pub q: Option<String>,
    pub status: Option<HostStatus>,
}

#[derive(Serialize, Debug)]
pub struct PaginationMetadataResponse {
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Serialize, Debug)]
pub struct GetAllHostResponse {
    pub data: Vec<HostResponse>,
    pub meta: PaginationMetadataResponse,
}

pub type GetHostByIdResponse = CreateHostResponse;

#[derive(Deserialize, Debug, Validate)]
pub struct UpdateHostMetadataRequest {
    #[validate(length(min = 3, max = 255))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[validate(length(min = 3, max = 255))]
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_endpoint: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct UpdateHostMetadataResponse {
    pub data: HostResponse,
}

#[derive(Serialize, Debug)]
pub struct HostObserveStatus {
    pub host_id: Uuid,
    pub status: HostStatus,
    pub observed_at: chrono::NaiveDateTime,
    pub docker_available: bool,
    pub compose_available: bool,
    pub docker_server_version: Option<String>,
    pub compose_version: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ObserveStatusResponse {
    pub host_id: Uuid,
    pub status: HostStatus,
    pub observed_at: chrono::NaiveDateTime,
    pub docker_available: bool,
    pub compose_available: bool,
    pub docker_server_version: Option<String>,
    pub compose_version: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GetHostProjectsRequest {
    pub cursor: Option<Uuid>,
    pub limit: Option<u16>,
}

#[derive(Serialize, Debug)]
pub struct GetHostProjectsResponse {
    pub data: Vec<ProjectResponse>,
    pub meta: PaginationMetadataResponse,
}
