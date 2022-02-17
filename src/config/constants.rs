use std::env;

lazy_static! {
    pub static ref BEARER: &'static str = "Bearer";
    pub static ref JWT_SECRET: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}
