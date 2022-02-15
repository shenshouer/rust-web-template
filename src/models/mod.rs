/// 数据库初始化相关与链接实例获取
pub mod db;

/// 用户模块数据操作实现
pub mod user;

/// 认证模块数据操作实现
pub mod credential;

use db::PgPool;
