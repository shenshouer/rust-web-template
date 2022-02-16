/// 数据库初始化相关与链接实例获取
pub mod db;
/// redis 缓存初始化相关
pub mod redis;

/// 用户模块数据操作实现
pub mod user;

/// 认证模块数据操作实现
pub mod credential;

/// token模块 采用redis缓存实现
pub mod token;

use db::PgPool;
