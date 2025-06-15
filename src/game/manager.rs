use anyhow::{Context, Result};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use tracing::{debug, trace};

use crate::Config;
use super::{SceneManager, GameData};

#[allow(unused)]
pub struct GameManager {
    exit: bool,
    data: Arc<Mutex<GameData>>,
    scenes: SceneManager,
}

impl GameManager {
    #[tracing::instrument(skip_all)]
    pub fn new(config: Config) -> Result<Self> {
        trace!("Creating a new game");

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
            debug!(
                assets_count = registry.amount(),
                path = ?assets_path.display(),
                "Assets registry created",
            );

            registry
        };

        let data = Arc::new(Mutex::new(GameData {
            config: config.clone(),
            assets,
        }));

        let scenes = SceneManager::new(data.clone());

        debug!("Game created");
        Ok(Self {
            data,
            scenes,
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

pub async fn start() -> Result<()> {
    debug!(version = ?env!("CARGO_PKG_VERSION"), "Launching game");

    let config = Config::new();
    config
        .check_if_exists_and_create()
        .context("Failed to check or create configuration")?;
    config.load().context("Failed to load configuration")?;

    let mut game = GameManager::new(config).context("Failed to create game instance")?;

    debug!("Entering game loop");
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
