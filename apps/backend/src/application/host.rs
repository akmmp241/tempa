use crate::http::host::dto::{
    CreateHostRequest, CreateHostResponse, GetAllHostRequest, GetAllHostResponse,
    GetHostProjectsRequest, GetHostProjectsResponse, ObserveStatusResponse,
    UpdateHostMetadataRequest, UpdateHostMetadataResponse,
};
use crate::ports::host_repository::HostRepository;
use sqlx::types::uuid;
use std::sync::Arc;

#[derive(Clone)]
pub struct HostService {
    host: Arc<dyn HostRepository>,
}

impl HostService {
    pub fn new(host: Arc<dyn HostRepository>) -> Self {
        Self { host }
    }

    pub async fn save(&self, req: &CreateHostRequest) -> anyhow::Result<CreateHostResponse> {
        todo!()
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
