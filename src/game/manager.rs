use anyhow::{Context, Result};
use egui::{DragValue, TopBottomPanel, menu};
use egui_plot::{Line, Plot};
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
            battle_settings: BattleSettings::default(),
        }));

        let mut scenes = SceneManager::new(Some(data.clone()))?;
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
                            if data.debug.plots {
                                ui.horizontal(|ui| {
                                    ui.label("Plot points:");
                                    ui.add(DragValue::new(&mut self.plot_points));
                                });
                            }
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
                            let available_width = ui.available_width();
                            let available_height = screen_height() / 3.0;

                            ui.horizontal(|ui| {
                                let (update_min, update_max) = self.update_plot.iter().fold(
                                    (f64::INFINITY, f64::NEG_INFINITY),
                                    |(min, max), point| (min.min(point[1]), max.max(point[1])),
                                );
                                let (render_min, render_max) = self.render_plot.iter().fold(
                                    (f64::INFINITY, f64::NEG_INFINITY),
                                    |(min, max), point| (min.min(point[1]), max.max(point[1])),
                                );

                                let update_points = rescale_y_range(
                                    &self.update_plot,
                                    update_min.min(render_min),
                                    update_max.max(render_max),
                                );
                                let render_points = rescale_y_range(
                                    &self.render_plot,
                                    update_min.min(render_min),
                                    update_max.max(render_max),
                                );

                                ui.allocate_ui([available_width, available_height].into(), |ui| {
                                    let update_line = Line::new("Update time", update_points);
                                    let render_line = Line::new("Render time", render_points);
                                    Plot::new("update_plot")
                                        .view_aspect(available_width / available_height)
                                        .label_formatter(|_, value| format!("{:.2} ms", value.y))
                                        .show(ui, |plot_ui| {
                                            plot_ui.line(update_line);
                                            plot_ui.line(render_line);
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

fn rescale_y_range(points: &[[f64; 2]], new_min: f64, new_max: f64) -> Vec<[f64; 2]> {
    let (min_y, max_y) = points
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), point| {
            (min.min(point[1]), max.max(point[1]))
        });

    let range_y = if max_y - min_y == 0.0 {
        1.0
    } else {
        max_y - min_y
    };

    points
        .iter()
        .map(|[x, y]| {
            let normalized_y = (y - min_y) / range_y;
            let scaled_y = normalized_y * (new_max - new_min) + new_min;
            [*x, scaled_y]
        })
        .collect()
}
