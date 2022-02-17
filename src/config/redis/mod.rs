use axum::async_trait;

pub mod redis;

#[async_trait]
pub trait RedisClient {
    async fn retrieve() -> Self;
}
