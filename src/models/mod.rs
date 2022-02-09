/// 数据库初始化相关与链接实例获取
mod db;

/// 用户模块数据操作实现
mod user;

use db::PgPool;
use std::collections::HashMap;

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
