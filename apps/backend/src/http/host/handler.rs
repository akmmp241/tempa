use crate::bootstrap::AppState;
use crate::http::dto::ApiResponse;
use crate::http::error::HttpError;
use crate::http::host::dto::{
    CreateHostRequest, CreateHostResponse, GetAllHostRequest, GetAllHostResponse,
    GetHostByIdResponse, GetHostProjectsRequest, GetHostProjectsResponse, ObserveStatusResponse,
    UpdateHostMetadataRequest, UpdateHostMetadataResponse,
};
use axum::Json;
use axum::extract::{Path, Query, State};
use http::StatusCode;
use sqlx::types::Uuid;

pub async fn create_host(
    State(state): State<AppState>,
    Json(payload): Json<CreateHostRequest>,
) -> Result<Json<ApiResponse<CreateHostResponse>>, HttpError> {
    todo!()
}

pub async fn get_hosts(
    State(state): State<AppState>,
    Query(query): Query<GetAllHostRequest>,
) -> Result<Json<ApiResponse<GetAllHostResponse>>, HttpError> {
    log::info!("hai");
    todo!()
}

pub async fn get_host_by_id(
    State(state): State<AppState>,
    Path(host_id): Path<Uuid>,
) -> Result<Json<ApiResponse<GetHostByIdResponse>>, HttpError> {
    todo!()
}

pub async fn update_host_metadata(
    State(state): State<AppState>,
    Path(host_id): Path<Uuid>,
    Json(payload): Json<UpdateHostMetadataRequest>,
) -> Result<Json<ApiResponse<UpdateHostMetadataResponse>>, HttpError> {
    todo!()
}

pub async fn delete_host(
    State(state): State<AppState>,
    Path(host_id): Path<Uuid>,
) -> Result<(StatusCode), HttpError> {
    todo!();

    Ok((StatusCode::NO_CONTENT))
}

pub async fn observe_host_status(
    State(state): State<AppState>,
    Path(host_id): Path<Uuid>,
) -> Result<Json<ApiResponse<ObserveStatusResponse>>, HttpError> {
    todo!()
}

pub async fn get_host_projects(
    State(state): State<AppState>,
    Path(host_id): Path<Uuid>,
    Query(query): Query<GetHostProjectsRequest>,
) -> Result<Json<ApiResponse<GetHostProjectsResponse>>, HttpError> {
    todo!()
}
