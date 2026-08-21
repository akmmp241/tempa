use crate::http::dto::{PaginationMetadataResponse, PaginationResponse};
use crate::http::error::HttpError;
use crate::http::host::dto::{
    CreateHostRequest, CreateHostResponse, GetAllHostRequest, GetAllHostResponse,
    GetHostProjectsRequest, GetHostProjectsResponse, HostResponse, ObserveStatusResponse,
    UpdateHostMetadataRequest, UpdateHostMetadataResponse,
};
use crate::ports::host_repository::{HostPosition, HostRepository};
use crate::ports::lib::PageRequest;
use crate::ports::project_repository::ProjectRepository;
use base64::{Engine, engine};
use chrono::Utc;
use domain::host::Host;
use engine::general_purpose;
use sqlx::types::uuid;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone)]
pub struct HostService {
    host: Arc<dyn HostRepository>,
    project: Arc<dyn ProjectRepository>,
}

impl HostService {
    pub fn new(host: Arc<dyn HostRepository>, project: Arc<dyn ProjectRepository>) -> Self {
        Self { host, project }
    }

    pub async fn save(&self, req: CreateHostRequest) -> Result<CreateHostResponse, HttpError> {
        req.validate()?;

        let host = Host::from(req);

        self.host
            .get_by("name", &host.id.to_string())
            .await?
            .is_some()
            .then(|| HttpError::BadRequest("duplicate name".to_string()));

        self.host.insert(&host).await?;

        // TODO: mekanisme buat ngecek status host online atau offline

        Ok(host.into())
    }

    pub async fn get_all(
        &self,
        req: GetAllHostRequest,
    ) -> Result<PaginationResponse<GetAllHostResponse>, HttpError> {
        let limit = req.limit.unwrap_or(10).clamp(1, i16::MAX);
        let q = req
            .q
            .and_then(|q| (!q.trim().is_empty()).then(|| q.trim().to_owned()));
        let status = req.status.map(|status| status.to_string());
        let cursor = req
            .cursor
            .map(|cursor| {
                let decoded = general_purpose::STANDARD.decode(cursor).map_err(|e| {
                    log::error!("failed occurred when decode base64 cursor:  {}", e);
                    HttpError::BadRequest("failed occurred when decode cursor".to_string())
                })?;
                let position = serde_json::from_slice::<HostPosition>(&decoded).map_err(|e| {
                    log::error!("failed occurred when deserialize position: {}", e);
                    HttpError::BadRequest("failed occurred when deserialize position".to_string())
                })?;
                Ok::<_, HttpError>(position)
            })
            .transpose()?;

        let mut hosts = self
            .host
            .get_all(PageRequest {
                after: cursor,
                limit: limit + 1,
                q,
                status,
            })
            .await?;

        let has_more = hosts.len() > limit as usize;
        if has_more {
            hosts.pop();
        }

        let next_cursor = has_more
            .then(|| hosts.last())
            .flatten()
            .map(|host| {
                let position = HostPosition {
                    id: host.id,
                    created_at: host.created_at.and_utc(),
                };
                serde_json::to_vec(&position).map(|value| general_purpose::STANDARD.encode(value))
            })
            .transpose()
            .unwrap();

        Ok((
            hosts.into_iter().map(Into::into).collect(),
            PaginationMetadataResponse {
                next_cursor,
                has_more,
            },
        ))
    }

    pub async fn get_by_id(&self, host_id: uuid::Uuid) -> Result<HostResponse, HttpError> {
        let host = self
            .host
            .get_by_id(&host_id.to_string())
            .await?
            .ok_or(HttpError::NotFound("host not found".to_string()))?;

        Ok(host.into())
    }

    pub async fn update_metadata(
        &self,
        host_id: &uuid::Uuid,
        req: UpdateHostMetadataRequest,
    ) -> Result<UpdateHostMetadataResponse, HttpError> {
        let mut host = self
            .host
            .get_by_id(&host_id.to_string())
            .await?
            .ok_or_else(|| HttpError::NotFound("host not found".to_string()))?;

        if let Some(name) = req.name {
            host.name = name;
        }

        if let Some(host_type) = req._type {
            host._type = host_type.into();
        }

        if let Some(docker_endpoint) = req.docker_endpoint {
            host.docker_endpoint = docker_endpoint;
        }

        log::debug!("host: {:?}", host);
        self.host.update(&host).await?;

        // TODO: implement mekanisme cek status host

        Ok(UpdateHostMetadataResponse { data: host.into() })
    }

    pub async fn delete(&self, host_id: &uuid::Uuid) -> Result<(), HttpError> {
        let host = self
            .host
            .get_by_id(&host_id.to_string())
            .await?
            .ok_or_else(|| HttpError::NotFound("host not found".to_string()))?;

        self.host.delete(&host.id.to_string()).await?;

        Ok(())
    }

    pub async fn status(&self, host_id: &uuid::Uuid) -> Result<ObserveStatusResponse, HttpError> {
        let host = self
            .host
            .get_by_id(&host_id.to_string())
            .await?
            .ok_or_else(|| HttpError::NotFound("host not found".to_string()))?;

        let status = ObserveStatusResponse {
            host_id: host.id,
            status: host.status,
            observed_at: Utc::now().naive_utc(),
            // TODO: implement cek status yang proper
            docker_available: false,
            compose_available: false,
            docker_server_version: None,
            compose_version: None,
        };

        Ok(status)
    }

    pub async fn list_projects(
        &self,
        host_id: &uuid::Uuid,
        req: GetHostProjectsRequest,
    ) -> Result<PaginationResponse<GetHostProjectsResponse>, HttpError> {
        self.host
            .get_by_id(&host_id.to_string())
            .await?
            .ok_or_else(|| HttpError::NotFound("host not found".to_string()))?;

        let limit = req.limit.unwrap_or(10).clamp(1, i16::MAX);
        let mut projects = self
            .project
            .get_by_host(host_id, req.cursor, (limit + 1) as i64)
            .await?;

        let has_more = projects.len() > limit as usize;
        if has_more {
            projects.pop();
        }

        let next_cursor = has_more
            .then(|| projects.last())
            .flatten()
            .map(|project| project.id.to_string());

        Ok((
            projects.into_iter().map(Into::into).collect(),
            PaginationMetadataResponse {
                next_cursor,
                has_more,
            },
        ))
    }
}
