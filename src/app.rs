use anyhow::{Context, Result};
use roketz::{handle_result_closure, Config, Game};
use std::sync::Arc;
use tracing::debug;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

pub struct App<'a> {
    config: Config,
    game: Option<Game<'a>>,
}

impl<'a> App<'a> {
    pub fn new(config: Config) -> Self {
        App { config, game: None }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    #[tracing::instrument(skip_all)]
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Winit Window")
            .with_inner_size(LogicalSize::new(
                self.config.graphics.width,
                self.config.graphics.height,
            ));

        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .context("Failed to create window")
                .unwrap(),
        );
        debug!(size = ?window.inner_size(), "Window created");

        handle_result_closure(|| {
            self.game = Some(
                Game::new(self.config.clone(), window.clone()).context("Failed to create game")?,
            );

            let game = self.game.as_mut().context("Game not initialized")?;
            game.window.request_redraw();
            debug!("First window redraw requested");

            Ok(())
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        handle_result_closure(|| {
            let game = self.game.as_mut().context("Game not initialized")?;
            game.handle_event(event_loop, event)?;
            Ok(())
        });
    }
}

pub fn create_and_run() -> Result<()> {
    let config = Config::new();
    config
        .check_if_exists_and_create()
        .context("Failed to check or create configuration")?;
    config.load().context("Failed to load configuration")?;

    let mut app = App::new(config);
    let event_loop = EventLoop::new().context("Failed to create event loop")?;
    event_loop
        .run_app(&mut app)
        .context("Failed to run application")?;

    Ok(())
}
