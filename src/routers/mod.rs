mod home;
mod users;

use axum::Router;

pub fn routers() -> Router {
    Router::new()
        .nest("/users", users::router())
        .nest("", home::router())
}
