#[macro_use]
extern crate lazy_static;

pub mod config;
mod dao;
mod dto;
mod errors;
pub mod models;
mod routers;
mod services;

use axum::Router;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

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
