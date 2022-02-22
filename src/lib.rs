#[macro_use]
extern crate lazy_static;

/// 配置解析、常量定义与数据库等连接池管理
pub mod config;
/// Data Access Object 数据访问层
mod dao;
/// Data Transfer Object 数据转换层
mod dto;
/// 错误定义
mod errors;
/// 数据模型定义
pub mod models;
/// 路由与handler等controller
mod routers;
/// controller 依赖的业务层实现
mod services;

use axum::Router;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

/// 路由入口
pub fn app(pg_pool: sqlx::PgPool) -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .into_inner();

    let pool = Arc::new(pg_pool);
    Router::new()
        .nest("/api/v1", routers::routers(pool.clone()))
        .layer(middleware_stack)
}
