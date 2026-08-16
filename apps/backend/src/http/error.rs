use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde_json::json;

pub enum HttpError {
    NotFound(String),
    BadRequest(String),
    InternalServerError,
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

            HttpError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "message": "Internal server error"
                })),
            )
                .into_response(),
        }
    }
}
