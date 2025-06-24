use anyhow::{Context, Result};
use egui::Window;
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::{
    game::{GameData, Scene},
    wrappers::{Bullet, Camera, CameraType, Player, Terrain},
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
    terrain: Terrain,
    first_camera: Camera,
    second_camera: Camera,
    first_player: Player,
    second_player: Player,
    bullets: Vec<Bullet>,
}

impl Scene for Battle {
    fn create(data: Option<Arc<Mutex<GameData>>>) -> Result<Self> {
        let data = data.context("Battle scene requires GameData")?.clone();

        let terrain_data = data
            .lock()
            .unwrap()
            .assets
            .get_asset::<assets::Terrain>("TestTerrain")
            .context("Failed to get terrain texture")?
            .clone();
        let terrain =
            Terrain::new(data.clone(), &terrain_data).context("Failed to create terrain")?;

        let first_player_spawn_point = vec2(
            terrain_data.player_one_x as f32,
            terrain_data.player_one_y as f32,
        );
        let second_player_spawn_point = vec2(
            terrain_data.player_two_x as f32,
            terrain_data.player_two_y as f32,
        );

        let mut battle = Self {
            data: data.clone(),
            ty: BattleType::Single,
            first_camera: Camera::new(CameraType::Global),
            second_camera: Camera::new(CameraType::Global),
            first_player: Player::builder(data.clone())
                .with_spawn_point(first_player_spawn_point)
                .with_gravity(50.0)
                .with_terrain_data(&terrain)
                .build(),
            second_player: Player::builder(data)
                .with_spawn_point(second_player_spawn_point)
                .with_gravity(50.0)
                .with_terrain_data(&terrain)
                .is_player_2(true)
                .build(),
            terrain,
            bullets: Vec::new(),
        };

        battle.ty = BattleType::MultiLeftRight;
        battle.first_camera.change_type(CameraType::Left);
        battle.second_camera.change_type(CameraType::Right);
        Ok(battle)
    }

    fn name(&self) -> &str {
        "Battle"
    }

    fn update(&mut self) {
        if is_key_pressed(KeyCode::T) {
            let player_pos = self.first_player.get_position();
            self.terrain
                .destruct(player_pos.x as u32, player_pos.y as u32, 10);
        }

        self.terrain.update();
        self.first_player
            .update(&mut self.terrain, &mut self.bullets);
        if self.ty != BattleType::Single {
            self.second_player
                .update(&mut self.terrain, &mut self.bullets);
        }

        for bullet in &mut self.bullets {
            bullet.update(&mut self.terrain, 50.0);
        }
        self.bullets.retain(|bullet| bullet.is_alive());

        self.first_camera.target = self.first_player.get_position();
        self.first_camera.update();
        self.second_camera.target = self.second_player.get_position();
        self.second_camera.update();
    }

    fn render(&self) {
        clear_background(DARKGRAY);

        match self.ty {
            BattleType::Single => {
                self.first_camera.set();
                self.terrain.draw();
                self.first_player.draw();
                for bullet in &self.bullets {
                    bullet.draw();
                }
            }
            BattleType::MultiTopBottom | BattleType::MultiLeftRight => {
                self.first_camera.set();
                self.terrain.draw();
                self.first_player.draw();
                self.second_player.draw();
                for bullet in &self.bullets {
                    bullet.draw();
                }

                self.second_camera.set();
                self.terrain.draw();
                self.first_player.draw();
                self.second_player.draw();
                for bullet in &self.bullets {
                    bullet.draw();
                }
            }
        }

        set_default_camera();

        match self.ty {
            BattleType::MultiTopBottom => {
                let screen_width = screen_width();
                let separator_y = screen_height() / 2.0;
                draw_line(0.0, separator_y, screen_width, separator_y, 2.0, WHITE);
            }
            BattleType::MultiLeftRight => {
                let screen_height = screen_height();
                let separator_x = screen_width() / 2.0;
                draw_line(separator_x, 0.0, separator_x, screen_height, 2.0, WHITE);
            }
            _ => {}
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        self.terrain.ui(ctx);
        self.first_player.ui(ctx);
        if self.ty != BattleType::Single {
            self.second_player.ui(ctx);
        }

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
