use axum::async_trait;

pub mod postgres;

/// 数据连接池获取接口
#[async_trait]
pub trait DbPool {
    async fn retrieve() -> Self;
}
