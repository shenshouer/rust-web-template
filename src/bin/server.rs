use anyhow::Result;
use cashbook::config;
use clap::Parser;
use dotenv::dotenv;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    use config::db::DbPool;

    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let config = config::env::ServerConfig::parse();

    let pool = sqlx::PgPool::retrieve().await;

    let addr = SocketAddr::from((config.serv_host, config.serv_port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(cashbook::app(pool).into_make_service())
        .await
        .unwrap();
    Ok(())
}
