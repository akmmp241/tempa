use crate::bootstrap::AppState;
use crate::http::dto::ApiResponse;
use crate::http::error::HttpError;
use crate::http::extractors::ValidateJson;
use crate::http::host::dto::{
    CreateHostRequest, CreateHostResponse, GetAllHostRequest, GetAllHostResponse,
    GetHostByIdResponse, GetHostProjectsRequest, GetHostProjectsResponse, ObserveStatusResponse,
    UpdateHostMetadataRequest, UpdateHostMetadataResponse,
};
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::types::Uuid;

pub async fn create_host(
    State(state): State<AppState>,
    Json(payload): Json<CreateHostRequest>,
) -> Result<Json<ApiResponse<CreateHostResponse>>, HttpError> {
    let res = state.host_service.save(payload).await?;

    Ok(Json(ApiResponse::success(
        "Host created successfully".to_string(),
        res,
    )))
}

pub async fn get_hosts(
    State(state): State<AppState>,
    Query(query): Query<GetAllHostRequest>,
) -> Result<Json<ApiResponse<GetAllHostResponse>>, HttpError> {
    let res = state.host_service.get_all(query).await?;

    Ok(Json(ApiResponse::success(
        "Hosts fetched successfully".to_string(),
        res,
    )))
}

pub async fn get_host_by_id(
    State(state): State<AppState>,
    Path(host_id): Path<Uuid>,
) -> Result<Json<ApiResponse<GetHostByIdResponse>>, HttpError> {
    let res = state.host_service.get_by_id(host_id).await?;

    Ok(Json(ApiResponse::success(
        "host fetched successfully".to_string(),
        res,
    )))
}

pub async fn update_host_metadata(
    State(state): State<AppState>,
    Path(host_id): Path<Uuid>,
    ValidateJson(payload): ValidateJson<UpdateHostMetadataRequest>,
) -> Result<Json<ApiResponse<UpdateHostMetadataResponse>>, HttpError> {
    let res = state
        .host_service
        .update_metadata(&host_id, payload)
        .await?;

    Ok(Json(ApiResponse::success(
        "host updated successfully".to_string(),
        res,
    )))
}

pub async fn delete_host(
    State(state): State<AppState>,
    Path(host_id): Path<Uuid>,
) -> Result<impl IntoResponse, HttpError> {
    state.host_service.delete(&host_id).await?;

    Ok(StatusCode::NO_CONTENT)
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
