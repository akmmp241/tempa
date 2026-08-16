use crate::http::error::HttpError;
use crate::http::host::dto::{
    CreateHostRequest, CreateHostResponse, GetAllHostRequest, GetAllHostResponse,
    GetHostProjectsRequest, GetHostProjectsResponse, ObserveStatusResponse,
    UpdateHostMetadataRequest, UpdateHostMetadataResponse,
};
use crate::ports::host_repository::HostRepository;
use domain::host::Host;
use sqlx::types::uuid;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone)]
pub struct HostService {
    host: Arc<dyn HostRepository>,
}

impl HostService {
    pub fn new(host: Arc<dyn HostRepository>) -> Self {
        Self { host }
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

        // TODO: mekanisme buat ngecek

        Ok(host.into())
    }

    pub async fn get_all(&self, req: &GetAllHostRequest) -> anyhow::Result<GetAllHostResponse> {
        todo!()
    }

    pub async fn get_by_id(&self, host_id: &uuid::Uuid) -> anyhow::Result<CreateHostResponse> {
        todo!()
    }

    pub async fn update_metadata(
        &self,
        host_id: &uuid::Uuid,
        req: UpdateHostMetadataRequest,
    ) -> anyhow::Result<UpdateHostMetadataResponse> {
        todo!()
    }

    pub async fn delete(&self, host_id: &uuid::Uuid) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn status(&self, host_id: &uuid::Uuid) -> anyhow::Result<ObserveStatusResponse> {
        todo!()
    }

    pub async fn list_projects(
        &self,
        host_id: &uuid::Uuid,
        req: GetHostProjectsRequest,
    ) -> anyhow::Result<GetHostProjectsResponse> {
        todo!()
    }
}
