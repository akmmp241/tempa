use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("not found: {0:?}")]
    NotFound(String),
    #[error("bad request: {0:?}")]
    BadRequest(String),
    #[error("Validation Error: {0:?}")]
    Validation(#[from] ValidationErrors),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal Server Error")]
    InternalServerError(#[from] anyhow::Error),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self {
            HttpError::NotFound(message) => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "message": message
                })),
            )
                .into_response(),
            HttpError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "message": message
                })),
            )
                .into_response(),
            HttpError::Validation(message) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "message": message
                })),
            )
                .into_response(),

            HttpError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "message": "Unauthorized"
                })),
            )
                .into_response(),
            HttpError::InternalServerError(msg) => {
                log::error!("internal server error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "success": false,
                        "message": "Internal server error"
                    })),
                )
            }
                .into_response(),
        }
    }
}
