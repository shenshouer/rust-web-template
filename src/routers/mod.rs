mod auth;
mod home;
mod users;

use axum::Router;
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub fn routers(pool: Arc<PgPool>, redis_client: Arc<redis::Client>) -> Router {
    Router::new()
        .nest("/users", users::router(pool.clone()))
        .nest("/auth", auth::router(pool.clone(), redis_client.clone()))
        .nest("", home::router())
}
