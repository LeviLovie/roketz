use anyhow::{Context, Result};
use macroquad::prelude::*;
use rasset::prelude::Registry;
use std::sync::Arc;
use tracing::debug;

use crate::{Config, handle_result};

pub struct Game {
    _assets: Arc<Registry>,
    _config: Config,
    exit: bool,
}

impl Game {
    #[tracing::instrument(skip_all)]
    pub fn new(config: Config) -> Result<Self> {
        let assets = {
            let exec_dir =
                std::env::current_exe().context("Failed to get current executable directory")?;
            let assets_path = exec_dir
                .parent()
                .context("Failed to get parent directory of executable")?
                .join(&config.assets);
            if !assets_path.exists() {
                return Err(anyhow::anyhow!(
                    "Assets file does not exist at {}",
                    assets_path.display()
                ));
            }

            let assets_binary = std::fs::read(&assets_path).context(format!(
                "Failed to read assets from {}",
                assets_path.display()
            ))?;

            let registry = assets::registry(assets_binary)?;

            Arc::new(registry)
        };

        debug!("Game created");
        Ok(Game {
            _assets: assets,
            _config: config,
            exit: false,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        if is_key_pressed(KeyCode::Escape) {
            self.exit = true;
            debug!("Exit requested");
        }

        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        Ok(())
    }
}

pub async fn gameloop() -> Result<()> {
    let config = Config::new();
    config
        .check_if_exists_and_create()
        .context("Failed to check or create configuration")?;
    config.load().context("Failed to load configuration")?;

    let mut game = Game::new(config).context("Failed to create game instance")?;
    loop {
        if game.exit {
            debug!("Quit requested, exiting game loop");
            break;
        }

        game.update().context("Failed to update game state")?;
        game.draw().context("Failed to draw game frame")?;
        next_frame().await;
    }

    Ok(())
}

pub async fn run() {
    handle_result(gameloop().await);
}
