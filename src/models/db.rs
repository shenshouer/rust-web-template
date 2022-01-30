use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use tokio::sync::OnceCell;

pub(super) type DbPool = Pool<Postgres>;

static ONCE: OnceCell<DbPool> = OnceCell::const_new();

/// 获取数据库连接池
pub(super) async fn pool() -> &'static DbPool {
    ONCE.get_or_init(get_pool).await
}

/// 获取数据库连接池
async fn get_pool() -> DbPool {
    let db_pool_size_str = env::var("DB_POOL_SIZE").unwrap_or(String::from("5"));
    let db_url = env::var("DB_URL").expect("无法从环境变量DB_URL中获取数据库信息");

    let pool = PgPoolOptions::new()
        .max_connections(db_pool_size_str.parse().unwrap())
        .connect(&db_url)
        .await
        .unwrap();
    pool
}
