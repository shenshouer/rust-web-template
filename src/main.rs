use anyhow::Result;
use axum::Router;
use cashbook::routers;
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let serv_addr = env::var("SERV_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let serv_port = env::var("SERV_PORT").unwrap_or("8080".to_string());

    let app = Router::new().nest("/", routers::routers());

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
