use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DataStore(#[from] sqlx::Error),
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error("empty fields")]
    EmptyFields(String),
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("wrong credentials")]
    WrongCredentials,
    #[error(transparent)]
    AxumTypedHeader(#[from] axum::extract::rejection::TypedHeaderRejection),
    #[error(transparent)]
    AxumExtension(#[from] axum::extract::rejection::ExtensionRejection),
    #[error("email: {0} is already taken")]
    DuplicateUserEmail(String),
}

impl Error {
    pub fn new_empty_fields_error(msg: String) -> Error {
        Error::EmptyFields(msg)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub type ApiError = (StatusCode, Json<Value>);
pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        let status = match err {
            Error::WrongCredentials => StatusCode::UNAUTHORIZED,
            Error::Validation(_) | Error::EmptyFields(_) | Error::DuplicateUserEmail(_) => {
                StatusCode::BAD_REQUEST
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let payload = json!({"ok": false, "error": err.to_string()});
        (status, Json(payload))
    }
}
