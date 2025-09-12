use utils::{
    crates::{
        tracing::Level,
        tracing_subscriber::{self, EnvFilter},
    },
    prelude::*,
};

pub fn init() {
    tracing_subscriber::fmt()
        .with_max_level(Level::WARN)
        .with_env_filter(EnvFilter::new("roketz=debug"))
        .init();
}

pub fn debug() {
    info!(
        version = env!("CARGO_PKG_VERSION"),
        "Launching {}",
        env!("CARGO_PKG_NAME")
    );
}
