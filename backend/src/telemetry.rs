use tracing_subscriber::EnvFilter;

pub fn init() {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::new("info")).init();
}