use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("data store error")]
    DataStoreError(#[from] sqlx::Error),
    #[error("unknown app error")]
    Unknown,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_message = match self {
            AppError::DataStoreError(err) => format!("Data store error {}", err),
            _ => "Other error".to_string(),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (StatusCode::OK, body).into_response()
    }
}
