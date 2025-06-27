use anyhow::{Context, Result};
use egui::{TopBottomPanel, menu};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info, trace};

use super::{GameData, SceneManager};
use crate::{config::Config, game::DebugState, scenes::BattleSettings};

#[allow(unused)]
pub struct GameManager {
    exit: bool,
    data: Arc<Mutex<GameData>>,
    scenes: SceneManager,
}

pub async fn start() -> Result<()> {
    info!(version = ?env!("CARGO_PKG_VERSION"), "Launching game");

    let config = Config::new();
    config
        .check_if_exists_and_create()
        .context("Failed to check or create configuration")?;
    config.load().context("Failed to load configuration")?;

    let mut game = GameManager::new(config).context("Failed to create game instance")?;

    info!("Entering game loop");
    loop {
        if game.exit {
            debug!("Exiting game loop");
            break;
        }

        game.update().context("Failed to update game state")?;
        game.render().context("Failed to draw game frame")?;
        next_frame().await;
    }

    trace!("Destroying game");
    game.destroy().context("Failed to destroy game manager")?;

    Ok(())
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
            trace!(
                assets_count = registry.amount(),
                path = ?assets_path.display(),
                "Assets registry created",
            );

            registry
        };

        let data = Arc::new(Mutex::new(GameData {
            config: config.clone(),
            assets,
            debug: DebugState::default(),
            battle_settings: BattleSettings::default(),
        }));

        let mut scenes = SceneManager::new(Some(data.clone()))?;
        crate::scenes::register(&mut scenes, data.clone()).context("Failed to register scenes")?;

        info!("Game created");
        Ok(Self {
            data,
            scenes,
            exit: false,
        })
    }

    fn get_data_mut(&mut self) -> Result<std::sync::MutexGuard<GameData>> {
        match self.data.lock() {
            Ok(data) => Ok(data),
            Err(e) => Err(anyhow::anyhow!("Failed to lock game data: {}", e)),
        }
    }

    pub fn update(&mut self) -> Result<()> {
        if self.scenes.should_quit() {
            self.exit = true;
            return Ok(());
        }

        if is_key_pressed(KeyCode::F3) {
            let mut data = self.get_data_mut()?;
            data.debug.enabled = !data.debug.enabled;
        }

        self.scenes.update()?;
        Ok(())
    }

    pub fn destroy(mut self) -> Result<()> {
        self.scenes.destroy()?;
        debug!("Game destroyed");
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        self.scenes.render()?;

        let mut result = Result::Ok(());
        egui_macroquad::ui(|ctx| {
            match self.scenes.ui(ctx) {
                Ok(_) => {}
                Err(e) => {
                    result = Err(e);
                    return;
                }
            }

            let mut should_exit = self.exit;
            {
                let mut data = match self.get_data_mut() {
                    Ok(data) => data,
                    Err(e) => {
                        result = Err(e);
                        return;
                    }
                };
                if data.debug.enabled {
                    TopBottomPanel::top("top_bar").show(ctx, |ui| {
                        menu::bar(ui, |ui| {
                            ui.menu_button("Debug", |ui| {
                                ui.label(format!("FPS: {:.2}", get_fps()));
                            });

                            ui.menu_button("Views", |ui| {
                                ui.checkbox(&mut data.debug.v_player, "Player");
                                ui.checkbox(&mut data.debug.v_terrain, "Terrain");
                                ui.checkbox(&mut data.debug.v_battle, "Battle");
                            });

                            ui.menu_button("Overlays", |ui| {
                                ui.checkbox(&mut data.debug.ol_bvh, "BVH");
                                ui.checkbox(&mut data.debug.ol_physics, "Physics");
                            });

                            ui.menu_button("Actions", |ui| {
                                ui.menu_button("Exit", |ui| {
                                    if ui.button("Gracefully").clicked() {
                                        should_exit = true;
                                    }

                                    if ui.button("Send SIGINT").clicked() {
                                        let self_pid = std::process::id();
                                        nix::sys::signal::kill(
                                            nix::unistd::Pid::from_raw(self_pid as i32),
                                            nix::sys::signal::Signal::SIGINT,
                                        )
                                        .unwrap_or_else(
                                            |e| {
                                                error!("Failed to send SIGINT: {}", e);
                                            },
                                        );
                                    }

                                    if ui.button("Send SIGTERM").clicked() {
                                        let self_pid = std::process::id();
                                        nix::sys::signal::kill(
                                            nix::unistd::Pid::from_raw(self_pid as i32),
                                            nix::sys::signal::Signal::SIGTERM,
                                        )
                                        .unwrap_or_else(
                                            |e| {
                                                error!("Failed to send SIGTERM: {}", e);
                                            },
                                        );
                                    }

                                    if ui.button("Send SIGKILL").clicked() {
                                        let self_pid = std::process::id();
                                        nix::sys::signal::kill(
                                            nix::unistd::Pid::from_raw(self_pid as i32),
                                            nix::sys::signal::Signal::SIGKILL,
                                        )
                                        .unwrap_or_else(
                                            |e| {
                                                error!("Failed to send SIGKILL: {}", e);
                                            },
                                        );
                                    }
                                });
                            });
                        });
                    });
                }
            }
            self.exit = should_exit;
        });
        egui_macroquad::draw();
        Ok(())
    }
}
