use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T, P = ()> {
    pub success: bool,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<P>,
}

impl<T, P> ApiResponse<T, P> {
    pub fn success(message: String, data: T) -> Self {
        Self {
            success: true,
            message,
            data: Some(data),
            meta: None,
        }
    }

    pub fn success_with_meta(message: String, data: T, meta: P) -> Self {
        Self {
            success: true,
            message,
            data: Some(data),
            meta: Some(meta),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct PaginationMetadataResponse {
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

pub type PaginationResponse<T> = (T, PaginationMetadataResponse);
