use anyhow::{Context, Result};
use egui::Window;
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::{
    game::{GameData, Scene},
    wrappers::{Camera, CameraType, Player, Terrain},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BattleType {
    Single,
    MultiTopBottom,
    MultiLeftRight,
}

pub struct Battle {
    data: Arc<Mutex<GameData>>,
    ty: BattleType,
    first_camera: Camera,
    second_camera: Camera,
    player: Player,
    terrain: Terrain,
}

impl Scene for Battle {
    fn create(data: Arc<Mutex<GameData>>) -> Result<Self> {
        let terrain_data = data
            .lock()
            .unwrap()
            .assets
            .get_asset::<assets::Terrain>("TestTerrain")
            .context("Failed to get terrain texture")?
            .clone();
        let terrain =
            Terrain::new(data.clone(), &terrain_data).context("Failed to create terrain")?;

        let mut player = Player::new(
            data.clone(),
            vec2(
                terrain_data.player_start_x as f32,
                terrain_data.player_start_y as f32,
            ),
        );
        player.gravity = 50.0;

        Ok(Self {
            data,
            ty: BattleType::Single,
            first_camera: Camera::new(CameraType::Global),
            second_camera: Camera::new(CameraType::Global),
            player,
            terrain,
        })
    }

    fn name(&self) -> &str {
        "Battle"
    }

    fn update(&mut self) {
        if is_key_pressed(KeyCode::T) {
            let player_pos = self.player.get_position();
            self.terrain.destruct(
                player_pos.x as u32,
                player_pos.y as u32,
                self.terrain.destruction_radius,
            );
        }

        self.first_camera.target = self.player.get_position();
        self.first_camera.update();
        self.second_camera.target = self.player.get_position();
        self.second_camera.update();

        self.player.update(&mut self.terrain);
        self.terrain.update();
    }

    fn render(&self) {
        clear_background(DARKGRAY);

        self.first_camera.set();
        self.terrain.draw(&self.first_camera);
        self.player.draw();

        // if self.ty != BattleType::Single {
        //     self.second_camera.set();
        //     self.terrain.draw(&self.second_camera);
        //     self.player.draw();
        // }

        set_default_camera();
    }

    fn ui(&mut self, ctx: &egui::Context) {
        self.player.ui(ctx);
        self.terrain.ui(ctx);

        if self.data.lock().unwrap().debug.v_battle {
            Window::new("Battle").show(ctx, |ui| {
                ui.label("Battle Scene");

                ui.separator();
                ui.label(format!("Type: {:?}", self.ty));
                if ui.button("Set to \"Single\"").clicked() {
                    self.ty = BattleType::Single;
                    self.first_camera.change_type(CameraType::Global);
                    self.second_camera.change_type(CameraType::Global);
                }
                if ui.button("Set to \"Multi Top-Bottom\"").clicked() {
                    self.ty = BattleType::MultiTopBottom;
                    self.first_camera.change_type(CameraType::Top);
                    self.second_camera.change_type(CameraType::Bottom);
                }
                if ui.button("Set to \"Multi Left-Right\"").clicked() {
                    self.ty = BattleType::MultiLeftRight;
                    self.first_camera.change_type(CameraType::Left);
                    self.second_camera.change_type(CameraType::Right);
                }
            });
        }
    }
}
