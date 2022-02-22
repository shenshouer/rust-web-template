use std::env;

lazy_static! {
    /// 认证token 类型
    pub static ref BEARER: &'static str = "Bearer";
    /// token加密字符串
    pub static ref JWT_SECRET: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}
