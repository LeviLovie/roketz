mod config;
mod game;
pub mod result;
pub mod scenes;
pub mod wrappers;

pub use config::Config;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[macroquad::main("Roketz")]
async fn main() {
    let file_appender =
        tracing_appender::rolling::daily("logs", format!("{}.log", env!("CARGO_PKG_NAME")));
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_target(true)
        .with_thread_names(true)
        .with_writer(file_writer);
    let filter = tracing_subscriber::EnvFilter::new("roketz=trace,warn");
    let registry = tracing_subscriber::registry().with(filter).with(file_layer);
    #[cfg(debug_assertions)]
    let registry = {
        let console_layer = tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .with_writer(std::io::stdout);
        registry.with(console_layer)
    };
    registry.init();

    game::run().await;
}
