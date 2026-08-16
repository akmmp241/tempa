use domain::host::HostStatus;
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
    pub _type: String,
    pub docker_endpoint: String,
}

#[derive(Serialize, Debug)]
pub struct HostResponse {
    pub id: Uuid,
    pub name: String,
    pub _type: String,
    pub docker_endpoint: String,
    pub status: HostStatus,
    pub project_count: u16,
    pub last_seen_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Debug)]
pub struct CreateHostResponse {
    pub data: HostResponse,
}

#[derive(Deserialize, Debug)]
pub struct GetAllHostRequest {
    cursor: Option<Uuid>,
    limit: Option<u16>,
    q: Option<String>,
    status: Option<HostStatus>,
}

#[derive(Serialize, Debug)]
pub struct PaginationMetadataResponse {
    pub next_cursor: Option<Uuid>,
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
    pub name: Option<String>,
    #[validate(length(min = 3, max = 255))]
    pub slug: Option<String>,
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
    pub data: HostObserveStatus,
}

#[derive(Deserialize, Debug)]
pub struct GetHostProjectsRequest {
    cursor: Option<Uuid>,
    limit: Option<u16>,
}

#[derive(Serialize, Debug)]
pub struct GetHostProjectsResponse {
    // TODO: change to ProjectResponse
    pub data: Vec<Uuid>,
    pub meta: PaginationMetadataResponse,
}
