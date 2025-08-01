#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![warn(clippy::mut_mut)]
#![warn(clippy::iter_nth)]

use helpers::error::HandleError;
use macroquad::window::Conf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn window_conf() -> Conf {
    Conf {
        window_title: "Roketz".into(),
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let log_dir = get_log_dir();
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir).handle("Failed to create log directory");
    }
    let file_appender =
        tracing_appender::rolling::daily(log_dir, format!("{}.log", env!("CARGO_PKG_NAME")));
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

    roketz::signals::install_signal_handler().handle("Failed to install signal handler");
    roketz::game::run().await;
}

fn get_log_dir() -> std::path::PathBuf {
    if let Some(dir) = dirs::data_local_dir() {
        dir.join("Roketz").join("logs")
    } else {
        std::path::PathBuf::from("logs")
    }
}
