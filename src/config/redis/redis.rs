use super::RedisClient;
use axum::async_trait;
use redis::Client;

use crate::config::env::RedisConfig;
use clap::Parser;

#[async_trait]
impl RedisClient for Client {
    async fn retrieve() -> Self {
        let config = RedisConfig::parse();
        let redis_url = format!("redis://{}:{}", config.redis_host, config.redis_port);
        redis::Client::open(redis_url).expect("Unable to connect to Redis")
    }
}
