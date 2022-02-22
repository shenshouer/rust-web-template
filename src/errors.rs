use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error;

// pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DataStoreError(#[from] sqlx::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("empty fields")]
    EmptyFields(String),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("wrong credentials")]
    WrongCredentials,
    #[error(transparent)]
    AxumTypedHeaderError(#[from] axum::extract::rejection::TypedHeaderRejection),
    #[error(transparent)]
    AxumExtensionError(#[from] axum::extract::rejection::ExtensionRejection),
    // #[error("password doesn't match")]
    // WrongPassword,
    // #[error("email is already taken")]
    // DuplicateUserEmail,
    // #[error("name is already taken")]
    // DuplicateUserName,
    // #[error("page limit not in range")]
    // WrongLimit(#[from] validator::ValidationError),
    // #[error("other error")]
    // Other(String)
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
            Error::ValidationError(_) | Error::EmptyFields(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let payload = json!({"ok": false, "error": err.to_string()});
        (status, Json(payload))
    }
}
