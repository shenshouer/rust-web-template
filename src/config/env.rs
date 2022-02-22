use std::net::IpAddr;

use clap::Parser;

/// http server 配置
#[derive(Debug, Parser)]
pub struct ServerConfig {
    #[clap(default_value = "127.0.0.1", env)]
    pub serv_host: IpAddr,
    #[clap(default_value = "8080", env)]
    pub serv_port: u16,
}

/// PostgresSQL 数据库配置
#[derive(Debug, Parser)]
pub struct PgConfig {
    #[clap(required = true, env)]
    pub pg_database: String,
    #[clap(default_value = "0.0.0.0", env)]
    pub pg_host: IpAddr,
    #[clap(default_value = "5432", env)]
    pub pg_port: u16,
    #[clap(default_value = "postgres", env)]
    pub pg_user: String,
    #[clap(default_value = "", env)]
    pub pg_password: String,
}
