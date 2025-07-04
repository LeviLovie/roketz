use anyhow::{Context, Result};
use egui::{TopBottomPanel, menu};
use macroquad::prelude::*;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tracing::{debug, error, info, trace};

use super::{GameData, SceneManager};
use crate::{config::Config, scenes::BattleSettings};

pub async fn start() -> Result<()> {
    info!(version = ?env!("CARGO_PKG_VERSION"), "Launching game");

    let config = Rc::new(RefCell::new(Config::new()));
    config
        .borrow()
        .check_if_exists_and_create()
        .context("Failed to check or create configuration")?;
    config
        .borrow_mut()
        .load()
        .context("Failed to load configuration")?;

    let mut game = GameManager::new(config.clone()).context("Failed to create game instance")?;

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

    config
        .borrow_mut()
        .save()
        .context("Failed to save configuration")?;

    Ok(())
}

#[allow(unused)]
pub struct GameManager {
    data: Rc<RefCell<GameData>>,
    scenes: SceneManager,
    exit: bool,
}

impl GameManager {
    #[tracing::instrument(skip_all)]
    pub fn new(config: Rc<RefCell<Config>>) -> Result<Self> {
        trace!("Creating a new game");

        let assets = {
            let assets_file_path = (std::env::current_exe()
                .context("Failed to get current executable directory")?)
            .parent()
            .context("Failed to get current exe parent dir")?
            .join("assets.rdss");
            let assets_file = assets_file_path
                .to_str()
                .context("Failed to convert assets path to string")?;
            let mut loader = rdss::Loader::new(assets_file);
            loader.load().context("Failed to load assets")?;
            loader
        };

        #[cfg(feature = "fmod")]
        let sound_engine = {
            let sound_engine = sound::SoundEngine::new(
                "assets/sound/Master.bank",
                vec!["assets/sound/Master.strings.bank"],
            )
            .context("Failed to initialize sound engine")?;
            sound_engine.list().context("Failed to list sound events")?;
            sound_engine
        };
        #[cfg(not(feature = "fmod"))]
        let sound_engine = ();

        let data = Rc::new(RefCell::new(GameData {
            sound_engine: Arc::new(Mutex::new(sound_engine)),
            config: config.clone(),
            assets,
            debug: false,
            battle_settings: BattleSettings::default(),
        }));

        let mut scenes = SceneManager::new(data.clone())?;
        crate::scenes::register(&mut scenes, data.clone()).context("Failed to register scenes")?;

        info!("Game created");
        Ok(Self {
            data,
            scenes,
            exit: false,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        if self.scenes.should_quit() {
            self.exit = true;
            return Ok(());
        }

        if is_key_pressed(KeyCode::F3) {
            let mut data = self.data.borrow_mut();
            data.debug = !data.debug;
        }

        self.scenes.update()?;
                #[cfg(feature = "fmod")]
                {
        match self.data.borrow_mut().sound_engine.lock() {
            Ok(mut sound_engine) => {
                    sound_engine
                        .update()
                        .context("Failed to update sound engine")?;
            }
            Err(e) => {
                error!("Failed to lock sound engine: {}", e);
            }
        }
                }
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        self.scenes.render()?;

        let mut result = Ok(());
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
                let data = self.data.borrow();
                if data.debug {
                    TopBottomPanel::top("top_bar").show(ctx, |ui| {
                        menu::bar(ui, |ui| {
                            ui.label(format!("FPS: {:.1}", get_fps()));

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

    pub fn destroy(mut self) -> Result<()> {
        self.scenes.destroy()?;
        debug!("Game destroyed");
        Ok(())
    }
}
