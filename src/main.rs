use anyhow::Result;
use axum::Router;
use cashbook::{models::db, routers};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let serv_addr = env::var("SERV_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let serv_port = env::var("SERV_PORT").unwrap_or("8080".to_string());

    let pool = Arc::new(db::get_pool().await);

    let app = Router::new().nest("/api/v1", routers::routers(pool.clone()));

    let addr = format!("{}:{}", serv_addr, serv_port)
        .as_str()
        .parse::<SocketAddr>()?;
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
