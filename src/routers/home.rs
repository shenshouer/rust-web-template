use axum::{routing::any, Router};

pub(crate) fn router() -> Router {
    Router::new().route("/", any(root))
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
