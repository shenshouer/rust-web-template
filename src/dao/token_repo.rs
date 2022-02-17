use crate::errors::AppError;
use axum::async_trait;
use redis::AsyncCommands;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct RedisTokenRepoImpl {
    client: Arc<redis::Client>,
}

impl RedisTokenRepoImpl {
    pub fn new(client: Arc<redis::Client>) -> Self {
        RedisTokenRepoImpl { client }
    }
}

use super::{Token, TokenRepo};

#[async_trait]
impl TokenRepo for RedisTokenRepoImpl {
    async fn generate_token(self: &Self) -> Token {
        Uuid::new_v4().to_string()
    }

    async fn save_token(self: &Self, token: &Token, username: &String) -> Result<(), AppError> {
        let client = &*self.client;
        let mut conn = client.get_async_connection().await?;
        let key = format!("token:{}", token);
        conn.set(key, username).await?;
        Ok(())
    }

    async fn get_username_by_token(self: &Self, token: &Token) -> Option<String> {
        let client = &*self.client;
        if let Ok(mut conn) = client.get_async_connection().await {
            let key = format!("token:{}", token);
            conn.get(key).await.ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_save_and_check_redis_token_repo() {
        let client = redis::Client::open("redis://localhost:6379").unwrap();
        let sut = RedisTokenRepoImpl {
            client: Arc::new(client),
        };

        let token = sut.generate_token().await;
        let username = "username".to_string();
        assert_eq!(None, sut.get_username_by_token(&token).await);
        assert_eq!((), sut.save_token(&token, &username).await.unwrap());
        assert_eq!(Some(username), sut.get_username_by_token(&token).await);
    }
}
