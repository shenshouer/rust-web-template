use async_trait::async_trait;
use serde::{Serialize, Deserialize};

pub type Token = String;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TokenRepo {
    async fn generate_token(&self) -> Token;
    async fn save_token(&self, token: &Token, username: &String) -> bool;
    async fn get_username_by_token(&self, token: &Token) -> Option<String>;
}