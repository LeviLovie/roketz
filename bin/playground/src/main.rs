use macroquad::prelude::*;
use tracing::{info};

#[macroquad::main("Playground")]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .init();
    info!(version = ?env!("CARGO_PKG_VERSION"), "Launching playground");

    info!("Entering main loop");
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
