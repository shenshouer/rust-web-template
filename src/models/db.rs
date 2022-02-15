pub(crate) use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
// use tokio::sync::OnceCell;

// static ONCE: OnceCell<PgPool> = OnceCell::const_new();

// /// 获取数据库连接池
// pub async fn pool() -> &'static PgPool {
//     ONCE.get_or_init(get_pool).await
// }

/// 获取数据库连接池
pub async fn get_pool() -> PgPool {
    let db_pool_size_str = env::var("DB_POOL_SIZE").unwrap_or(String::from("5"));
    let db_url = env::var("DB_URL").expect("无法从环境变量DB_URL中获取数据库信息");

    let pool = PgPoolOptions::new()
        .max_connections(db_pool_size_str.parse().unwrap())
        .connect(&db_url)
        .await
        .unwrap();
    pool
}
