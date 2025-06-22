use anyhow::{Context, Result};
use egui::{DragValue, TopBottomPanel, menu};
use egui_plot::{Line, Plot, PlotPoints};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info, trace};

use super::{GameData, SceneManager};
use crate::{config::Config, game::DebugState};

#[allow(unused)]
pub struct GameManager {
    exit: bool,
    data: Arc<Mutex<GameData>>,
    scenes: SceneManager,
    start: std::time::Instant,
    current_frame: f64,
    last_frame: std::time::Instant,
    plot_points: u32,
    update_plot: Vec<[f64; 2]>,
    render_plot: Vec<[f64; 2]>,
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
        }));

        let mut scenes = SceneManager::new(data.clone())?;
        crate::scenes::register(&mut scenes, data.clone()).context("Failed to register scenes")?;

        info!("Game created");
        Ok(Self {
            data,
            scenes,
            exit: false,
            start: std::time::Instant::now(),
            current_frame: 0.0,
            last_frame: std::time::Instant::now(),
            plot_points: 250,
            update_plot: Vec::new(),
            render_plot: Vec::new(),
        })
    }

    pub fn update(&mut self) -> Result<()> {
        let update_start = std::time::Instant::now();
        let now = std::time::Instant::now();
        self.current_frame = now.duration_since(self.start).as_secs_f64() * 1000.0;

        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            trace!("Exit requested");
            self.exit = true;
        }

        if is_key_pressed(KeyCode::F3) {
            let mut data = self.data.lock().unwrap();
            data.debug.enabled = !data.debug.enabled;
        }

        self.scenes.update()?;

        let update_duration = update_start.elapsed().as_micros() as f64 / 1000.0;
        self.update_plot.push([self.current_frame, update_duration]);
        while self.update_plot.len() > self.plot_points as usize {
            self.update_plot.remove(0);
        }
        Ok(())
    }

    pub fn destroy(mut self) -> Result<()> {
        self.scenes.destroy();
        debug!("Game destroyed");
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        let render_start = std::time::Instant::now();

        self.scenes.render()?;

        egui_macroquad::ui(|ctx| {
            self.scenes.ui(ctx);

            if self.data.lock().unwrap().debug.enabled {
                let mut data = self.data.lock().unwrap();
                TopBottomPanel::top("top_bar").show(ctx, |ui| {
                    menu::bar(ui, |ui| {
                        ui.menu_button("Debug", |ui| {
                            ui.label(format!("FPS: {:.2}", get_fps()));
                            ui.checkbox(&mut data.debug.plots, "Performance");
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
                                    self.exit = true;
                                }

                                if ui.button("Send SIGINT").clicked() {
                                    let self_pid = std::process::id();
                                    nix::sys::signal::kill(
                                        nix::unistd::Pid::from_raw(self_pid as i32),
                                        nix::sys::signal::Signal::SIGINT,
                                    )
                                    .unwrap_or_else(|e| {
                                        error!("Failed to send SIGINT: {}", e);
                                    });
                                }

                                if ui.button("Send SIGTERM").clicked() {
                                    let self_pid = std::process::id();
                                    nix::sys::signal::kill(
                                        nix::unistd::Pid::from_raw(self_pid as i32),
                                        nix::sys::signal::Signal::SIGTERM,
                                    )
                                    .unwrap_or_else(|e| {
                                        error!("Failed to send SIGTERM: {}", e);
                                    });
                                }

                                if ui.button("Send SIGKILL").clicked() {
                                    let self_pid = std::process::id();
                                    nix::sys::signal::kill(
                                        nix::unistd::Pid::from_raw(self_pid as i32),
                                        nix::sys::signal::Signal::SIGKILL,
                                    )
                                    .unwrap_or_else(|e| {
                                        error!("Failed to send SIGKILL: {}", e);
                                    });
                                }
                            });
                        });
                    });
                });

                if data.debug.plots {
                    TopBottomPanel::bottom("Performance")
                        .max_height(300.0)
                        .show(ctx, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Plot points:");
                                ui.add(DragValue::new(&mut self.plot_points));
                            });

                            let available_width = ui.available_width();
                            ui.horizontal(|ui| {
                                ui.allocate_ui([available_width / 2.0 - 25.0, 200.0].into(), |ui| {
                                    ui.vertical(|ui| {
                                        let update_points: PlotPoints =
                                            self.update_plot.clone().into();
                                        let update_line = Line::new("Update time", update_points);
                                        ui.label("Update time (ms)");
                                        Plot::new("update_plot")
                                            .view_aspect(2.0)
                                            .label_formatter(|_, value| {
                                                format!("{:.2} ms", value.y)
                                            })
                                            .show(ui, |plot_ui| {
                                                plot_ui.line(update_line);
                                            });
                                    });
                                });

                                ui.allocate_ui([available_width / 2.0 - 25.0, 200.0].into(), |ui| {
                                    ui.vertical(|ui| {
                                        let render_points: PlotPoints =
                                            self.render_plot.clone().into();
                                        let render_line = Line::new("Render time", render_points);
                                        ui.label("Render time (ms)");
                                        Plot::new("render_plot")
                                            .view_aspect(2.0)
                                            .label_formatter(|_, value| {
                                                format!("{:.2} ms", value.y)
                                            })
                                            .show(ui, |plot_ui| {
                                                plot_ui.line(render_line);
                                            });
                                    });
                                });
                            });
                        });
                }
            }
        });
        egui_macroquad::draw();

        let render_duration = render_start.elapsed().as_micros() as f64 / 1000.0;
        self.render_plot.push([self.current_frame, render_duration]);
        while self.render_plot.len() > self.plot_points as usize {
            self.render_plot.remove(0);
        }
        Ok(())
    }
}
