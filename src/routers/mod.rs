mod home;
mod users;

use axum::Router;
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub fn routers(pool: Arc<PgPool>) -> Router {
    Router::new()
        .nest("/users", users::router(pool.clone()))
        .nest("", home::router())
}
