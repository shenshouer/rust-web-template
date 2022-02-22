use crate::models::user::User;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginInput {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct AuthPayload {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
}
