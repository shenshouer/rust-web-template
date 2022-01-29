use axum::{extract::Path, routing::get, Router};
use std::collections::HashMap;

async fn users_get(Path(params): Path<HashMap<String, String>>) {
    // Both `version` and `id` were captured even though `users_api` only
    // explicitly captures `id`.
    let _version = params.get("version");
    let _id = params.get("id");
}

#[tokio::main]
async fn main() {
    let users_api = Router::new().route("/users/:id", get(users_get));

    let app = Router::new().nest("/:version/api", users_api);

    axum::Server::bind(&"".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
