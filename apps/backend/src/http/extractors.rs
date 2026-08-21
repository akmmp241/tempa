use std::collections::HashMap;

use axum::{
    Json,
    extract::FromRequest,
    response::{IntoResponse, Response},
};
use http::StatusCode;
use serde::{Serialize, de::DeserializeOwned};
use validator::{Validate, ValidationErrors};

pub struct ValidateJson<T>(pub T);

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    errors: HashMap<String, Vec<String>>,
}

impl<S, T> FromRequest<S> for ValidateJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Response;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| {
                (
                    rejection.status(),
                    Json(ErrorResponse {
                        message: "invalid request body".to_string(),
                        errors: HashMap::from([("body".to_string(), vec![rejection.body_text()])]),
                    }),
                )
                    .into_response()
            })?;

        value.validate().map_err(validation_response)?;

        Ok(ValidateJson(value))
    }
}

fn validation_response(errors: ValidationErrors) -> Response {
    let mut fields: HashMap<String, Vec<String>> = HashMap::new();

    for (field, field_errors) in errors.field_errors() {
        let messages = field_errors
            .iter()
            .map(|error| {
                error
                    .message
                    .as_ref()
                    .map(|message| message.to_string())
                    .unwrap_or_else(|| format!("Invalid value: {}", field))
            })
            .collect();

        fields.insert(field.to_string(), messages);
    }

    (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(ErrorResponse {
            message: "Validation failed".to_string(),
            errors: fields,
        }),
    )
        .into_response()
}
