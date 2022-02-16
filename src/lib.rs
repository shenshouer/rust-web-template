mod errors;
pub mod models;
pub mod routers;
mod services;

#[cfg(test)]
mod tests {
    #[test]
    fn init_log() {
        tracing_subscriber::fmt::init();
    }
}
