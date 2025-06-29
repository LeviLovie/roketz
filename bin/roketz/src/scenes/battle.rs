use anyhow::{Context, Result};
use bevy_ecs::prelude::*;
use egui::{Align, CentralPanel, Layout, RichText};
use macroquad::prelude::*;
use rapier2d::prelude::*;
use std::{cell::RefCell, rc::Rc};

use crate::{
    camera::{Camera, CameraType},
    game::{GameData, Scene},
};
use ecs::{
    cs::{
        Player, RigidCollider, Terrain, Transform, disable_camera, draw_bullets, draw_players,
        draw_terrain, render_colliders, transfer_colliders, ui_players, update_bullets,
        update_players, update_terrain,
    },
    r::{DT, Debug, PhysicsWorld, init_physics, step_physics},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BattleType {
    Single,
    MultiTopBottom,
    MultiLeftRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleSettings {
    pub ty: BattleType,
}

impl Default for BattleSettings {
    fn default() -> Self {
        Self {
            ty: BattleType::Single,
        }
    }
}

pub struct Battle {
    data: Rc<RefCell<GameData>>,
    transfer: Option<String>,
    ty: BattleType,
    is_paused: bool,
    world: World,
    update: Schedule,
    draw: Schedule,
    cameras: Vec<Camera>,
}

impl Scene for Battle {
    fn name(&self) -> &str {
        "Battle"
    }

    fn should_transfer(&self) -> Option<String> {
        self.transfer.clone()
    }

    fn create(data: Rc<RefCell<GameData>>) -> Result<Self> {
        let (terrain_data, ty) = {
            let terrain_data = data
                .borrow()
                .assets
                .get_asset::<assets::Terrain>("TestTerrain")
                .context("Failed to get terrain texture")?
                .clone();
            let ty = data.borrow().battle_settings.ty;
            (terrain_data, ty)
        };

        let mut world = World::new();
        let mut init = Schedule::default();
        let mut update = Schedule::default();
        let mut draw = Schedule::default();

        world.insert_resource(DT(0.0));
        world.insert_resource(Debug::default());

        init.add_systems(init_physics);

        init.run(&mut world);

        world.spawn((Terrain::new(&terrain_data)?,));

        update.add_systems(
            (
                (update_terrain, update_bullets),
                update_players,
                step_physics,
                transfer_colliders,
            )
                .chain(),
        );

        draw.add_systems(
            (
                draw_terrain,
                draw_bullets,
                draw_players,
                render_colliders,
                disable_camera,
                ui_players,
            )
                .chain(),
        );

        let mut battle = Self {
            data,
            transfer: None,
            ty,
            is_paused: false,
            world,
            update,
            draw,
            cameras: Vec::new(),
        };
        battle.respawn_players()?;
        Ok(battle)
    }

    fn reload(&mut self) -> Result<()> {
        self.transfer = None;
        self.is_paused = false;
        let new_ty = self.data.borrow().battle_settings.ty;
        if self.ty != new_ty {
            self.ty = new_ty;
            self.respawn_players()?;
        }
        Ok(())
    }

    fn update(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.is_paused = !self.is_paused;
        }

        if self.is_paused {
            self.update_paused();
            return;
        }

        self.world.resource_mut::<DT>().0 = get_frame_time();

        self.update.run(&mut self.world);

        self.update_camera_types();
        for camera in self.cameras.iter_mut() {
            camera.update(&mut self.world);
        }
    }

    fn render(&mut self) {
        clear_background(BLACK);

        for camera in self.cameras.iter() {
            camera.set();
            self.draw.run(&mut self.world);
        }

        self.render_separator();

        if self.is_paused {
            self.render_paused();
        }
    }

    fn ui(&mut self, ctx: &egui::Context) -> Result<()> {
        if self.is_paused {
            self.ui_paused(ctx);
        }

        if self.data.borrow().debug {
            egui::Window::new("Debug").show(ctx, |ui| {
                ui.collapsing("Overlays", |ui| {
                    let mut overlays = self.world.resource_mut::<Debug>();
                    ui.checkbox(&mut overlays.o_physics, "Physics");
                });
            });
        }
        Ok(())
    }
}

impl Battle {
    fn update_camera_types(&mut self) {
        match self.cameras.len() {
            1 => {
                self.cameras[0].change_type(CameraType::Global);
            }
            2 => match self.ty {
                BattleType::Single => {
                    self.cameras[0].change_type(CameraType::Global);
                    self.cameras.remove(1);
                }
                BattleType::MultiTopBottom => {
                    self.cameras[0].change_type(CameraType::Top);
                    self.cameras[1].change_type(CameraType::Bottom);
                }
                BattleType::MultiLeftRight => {
                    self.cameras[0].change_type(CameraType::Left);
                    self.cameras[1].change_type(CameraType::Right);
                }
            },
            _ => {}
        }
    }

    fn update_paused(&self) {}

    fn render_paused(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        draw_rectangle(
            0.0,
            0.0,
            screen_width,
            screen_height,
            Color::from_rgba(0, 0, 0, 100),
        );
    }

    fn ui_paused(&mut self, ctx: &egui::Context) {
        ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_black_alpha(0),
            panel_fill: egui::Color32::from_black_alpha(0),
            ..Default::default()
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ui.add_space(screen_height() / 6.0);
                ui.label(RichText::new("Game paused").size(32.0));
                ui.add_space(screen_height() / 12.0);

                if ui.button(RichText::new("Resume").size(24.0)).clicked() {
                    self.is_paused = false;
                }
                if ui
                    .button(RichText::new("Quit to menu").size(24.0))
                    .clicked()
                {
                    self.transfer = Some("Menu".to_string());
                }
                if ui
                    .button(RichText::new("Exit to system").size(24.0))
                    .clicked()
                {
                    self.transfer = Some("__quit".to_string());
                }
            });
        });
        ctx.set_visuals(egui::Visuals::default());
    }

    fn spawn_player(&mut self, spawn_pos: Vec2, color: Color, is_player_1: bool) -> Entity {
        let mut physics = self.world.resource_mut::<PhysicsWorld>();
        let player = (
            Player::new(color, is_player_1),
            Transform::from_pos(spawn_pos),
            RigidCollider::dynamic(
                &mut physics,
                ColliderBuilder::ball(3.0).build(),
                vector![spawn_pos.x, spawn_pos.y],
                vector![0.0, 0.0],
                0.0,
            ),
        );
        self.world.spawn(player).id()
    }

    fn respawn_players(&mut self) -> Result<()> {
        for camera in self.cameras.iter_mut() {
            self.world.despawn(camera.id);
        }
        self.cameras.clear();

        let terrain_data = self
            .data
            .borrow()
            .assets
            .get_asset::<assets::Terrain>("TestTerrain")
            .context("Failed to get terrain texture")?
            .clone();
        let first_player_spawn_point = vec2(
            terrain_data.player_one_x as f32,
            terrain_data.player_one_y as f32,
        );
        let second_player_spawn_point = vec2(
            terrain_data.player_two_x as f32,
            terrain_data.player_two_y as f32,
        );

        let player_id = self.spawn_player(
            first_player_spawn_point,
            Color::from_rgba(66, 233, 245, 255),
            true,
        );
        self.cameras.push(Camera::new(player_id));

        if self.ty != BattleType::Single {
            let second_player_id = self.spawn_player(
                second_player_spawn_point,
                Color::from_rgba(235, 107, 52, 255),
                false,
            );
            self.cameras.push(Camera::new(second_player_id));
        }

        Ok(())
    }

    fn render_separator(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let separator_color = Color::from_rgba(100, 100, 100, 255);

        match self.ty {
            BattleType::Single => {}
            BattleType::MultiTopBottom => {
                draw_line(
                    0.0,
                    screen_height / 2.0,
                    screen_width,
                    screen_height / 2.0,
                    3.0,
                    separator_color,
                );
            }
            BattleType::MultiLeftRight => {
                draw_line(
                    screen_width / 2.0,
                    0.0,
                    screen_width / 2.0,
                    screen_height,
                    3.0,
                    separator_color,
                );
            }
        }
    }
}
