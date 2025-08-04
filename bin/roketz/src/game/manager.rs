use anyhow::{Context, Result};
use egui::{TopBottomPanel, menu};
use helpers::error::HandleError;
use macroquad::prelude::*;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tracing::{debug, error, info, trace};

use super::{GameData, SceneManager};
use crate::{
    ecs::r::BattleSettings, resolutions::Resolutions, settings::Settings, sprites::Sprites,
};

pub async fn start() -> Result<()> {
    info!(version = ?env!("CARGO_PKG_VERSION"), "Launching game");

    let settings_raw = Arc::new(Mutex::new(Settings::new()));
    settings_raw
        .lock()
        .handle("Failed to lock settings mutex")
        .check_if_exists_and_create()
        .context("Failed to check or create settingsuration")?;
    let settings = settings_raw
        .lock()
        .handle("Failed to lock settings mutex")
        .load()
        .context("Failed to load settingsuration")?;

    request_new_screen_size(settings.window.width as f32, settings.window.height as f32);
    set_fullscreen(settings.window.fullscreen);

    let mut game =
        GameManager::new(settings_raw.clone()).context("Failed to create game instance")?;

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

    settings_raw
        .lock()
        .handle("Failed to lock settings mutex")
        .save()
        .context("Failed to save settingsuration")?;

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
    pub fn new(settings: Arc<Mutex<Settings>>) -> Result<Self> {
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
            for file in loader.files() {
                trace!("Assets: {}", file);
            }
            Arc::new(Mutex::new(loader))
        };

        #[cfg(feature = "fmod")]
        let sound_engine = {
            let sound_engine = sound::SoundEngine::new(
                "assets/sound/Master.bank",
                vec!["assets/sound/Master.strings.bank"],
            )
            .context("Failed to initialize sound engine")?;
            sound_engine
        };
        #[cfg(not(feature = "fmod"))]
        let sound_engine = {
            error!("FMOD feature is not enabled. Compile with the 'fmod' feature.");
            crate::ecs::r::SoundEngine::new("", vec![])
        };

        let sprites = Sprites::load(assets.clone()).context("Failed to load sprites")?;
        let resolutions =
            Resolutions::load(assets.clone()).context("Failed to load resolutions")?;

        let data = Rc::new(RefCell::new(GameData {
            settings: settings.clone(),
            assets,
            sprites: Arc::new(Mutex::new(sprites)),
            resolutions,
            sound: Arc::new(Mutex::new(sound_engine)),
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
            use helpers::error::HandleError;

            self.data
                .borrow_mut()
                .sound
                .lock()
                .handle("Faield to lock sound mutex")
                .update()
                .context("Failed to update sound engine")?;
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
