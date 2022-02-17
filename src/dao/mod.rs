mod credential_repo;
mod token_repo;
pub(crate) mod user_repo;

// re export to public
pub(crate) use credential_repo::CredentialRepoImpl;
pub(crate) use token_repo::RedisTokenRepoImpl;

use crate::{errors::AppError, models::credential::Credential};
use axum::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CredentialRepo {
    async fn save_credential(&self, credential: &Credential) -> Result<bool, AppError>;
    async fn is_credential_exists(&self, credential: &Credential) -> bool;
}

pub type Token = String;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TokenRepo {
    async fn generate_token(&self) -> Token;
    async fn save_token(&self, token: &Token, username: &String) -> Result<(), AppError>;
    async fn get_username_by_token(&self, token: &Token) -> Option<String>;
}
