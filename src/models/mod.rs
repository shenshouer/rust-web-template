mod db;
mod user;

use std::collections::HashMap;

use db::{pool, DbPool};

/// 表特征
/// 所有的数据实例都需要实现此特征
trait Table {
    /// 获取实例对应的表明称
    fn table_name(&self) -> &str;
}

/// DB查询请求选项
trait DbQueryOption {
    fn option<T: Copy>(self) -> HashMap<String, T>;
}
