use anyhow::{Context, Result, bail};
use bevy_ecs::prelude::*;
use egui::{Align, CentralPanel, Layout, RichText};
use macroquad::prelude::*;
use rapier2d::prelude::*;
use std::{cell::RefCell, rc::Rc};

use crate::ecs::{
    cs::{
        Player, RigidCollider, Terrain, Transform, disable_camera, draw_bullets, draw_players,
        draw_terrain, handle_bullet_terrain_collisions, handle_player_bullet_collisions,
        init_terrain, render_colliders, transfer_colliders, ui_players, update_bullets,
        update_explosions, update_players, update_terrain,
    },
    r::{
        DT, Debug, PhysicsWorld, Sound, add_assets, collect_collisions, init_collisions,
        init_debug, init_dt, init_physics, init_thrust_sound, step_physics, update_thrust_sound,
    },
};
use crate::{
    camera::{Camera, CameraType},
    game::{GameData, Scene},
    scenes::{SCENE_MENU, SCENE_QUIT},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BattleType {
    Single,
    MultiTopBottom,
    MultiLeftRight,
}

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct BattleSettings {
    pub ty: BattleType,
    pub map: Option<String>,
}

impl Default for BattleSettings {
    fn default() -> Self {
        Self {
            ty: BattleType::Single,
            map: None,
        }
    }
}

pub const SCENE_BATTLE: &str = "Battle";
pub struct Battle {
    data: Rc<RefCell<GameData>>,
    transfer: Option<String>,
    ty: BattleType,
    is_paused: bool,
    dt_history: Vec<f32>,
    world: World,
    update: Schedule,
    draw: Schedule,
    cameras: Vec<Camera>,
}

impl Scene for Battle {
    fn name(&self) -> &str {
        SCENE_BATTLE
    }

    fn should_transfer(&self) -> Option<String> {
        self.transfer.clone()
    }

    fn create(data: Rc<RefCell<GameData>>) -> Result<Self> {
        Ok(Self {
            data: data.clone(),
            transfer: None,
            ty: data.borrow().battle_settings.ty,
            is_paused: false,
            dt_history: Vec::new(),
            world: World::new(),
            update: Schedule::default(),
            draw: Schedule::default(),
            cameras: Vec::new(),
        })
    }

    fn reload(&mut self) -> Result<()> {
        let mut world = World::new();
        let mut init = Schedule::default();
        let mut update = Schedule::default();
        let mut draw = Schedule::default();

        world.insert_resource(Sound::new(self.data.borrow().sound.clone()));
        world.insert_resource(self.data.borrow().battle_settings.clone());
        add_assets(&mut world, self.data.borrow().assets.clone());

        init.add_systems(
            (
                init_collisions,
                init_physics,
                init_thrust_sound,
                init_dt,
                init_debug,
                init_terrain,
            )
                .chain(),
        );

        init.run(&mut world);

        update.add_systems(
            (
                collect_collisions,
                (update_terrain, update_bullets),
                update_players,
                (step_physics, update_explosions, update_thrust_sound),
                (
                    transfer_colliders,
                    handle_bullet_terrain_collisions,
                    handle_player_bullet_collisions,
                ),
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

        self.world = world;
        self.update = update;
        self.draw = draw;

        self.transfer = None;
        self.is_paused = false;
        self.ty = self.data.borrow().battle_settings.ty;
        self.respawn_players()
            .context("Failed to respawn players")?;
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

        if self.world.resource_mut::<Debug>().p_dt {
            self.render_dt();
        }
    }

    fn ui(&mut self, ctx: &egui::Context) -> Result<()> {
        if self.is_paused {
            self.ui_paused(ctx);
        }

        if self.data.borrow().debug {
            egui::Window::new("Debug").show(ctx, |ui| {
                ui.collapsing("Overlays", |ui| {
                    let mut debug = self.world.resource_mut::<Debug>();
                    ui.checkbox(&mut debug.o_physics, "Physics");
                    ui.checkbox(&mut debug.o_bvh, "BVH");
                });

                ui.collapsing("Profiling", |ui| {
                    let mut debug = self.world.resource_mut::<Debug>();
                    ui.checkbox(&mut debug.p_dt, "Delta time");
                });
            });
        }
        Ok(())
    }
}

impl Battle {
    fn play_click_sound(&self) {
        #[cfg(feature = "fmod")]
        {
            match self.world.get_resource::<Sound>() {
                Some(sound) => {
                    sound
                        .borrow()
                        .play(sound::bindings::EVENT_UI_CLICK)
                        .unwrap_or_else(|e| error!("Error playing click sound: {}", e));
                }
                None => {
                    error!("Failed to get sound resource");
                }
            }
        }
    }

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
                    self.play_click_sound();
                    self.is_paused = false;
                }
                if ui
                    .button(RichText::new("Quit to menu").size(24.0))
                    .clicked()
                {
                    self.play_click_sound();
                    self.transfer = Some(SCENE_MENU.to_string());
                }
                if ui
                    .button(RichText::new("Exit to system").size(24.0))
                    .clicked()
                {
                    self.play_click_sound();
                    self.transfer = Some(SCENE_QUIT.to_string());
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
                ColliderBuilder::ball(3.0),
                vector![spawn_pos.x, spawn_pos.y],
                vector![0.0, 0.0],
                0.0,
            ),
        );
        self.world.spawn(player).id()
    }

    fn respawn_players(&mut self) -> Result<()> {
        for camera in self.cameras.iter_mut() {
            let _ = self.world.try_despawn(camera.id);
        }
        self.cameras.clear();

        let spawns = {
            self.world
                .query::<&Terrain>()
                .single(&self.world)
                .context("Failed to get terrain")?
                .spawns
                .clone()
        };

        if spawns.is_empty() {
            bail!("No spawn points found in the terrain");
        }
        let player_id = self.spawn_player(spawns[0], Color::from_rgba(66, 233, 245, 255), true);
        self.cameras.push(Camera::new(player_id));

        if self.ty != BattleType::Single {
            if spawns.len() < 2 {
                bail!("Not enough spawn points for two players");
            }
            let second_player_id =
                self.spawn_player(spawns[1], Color::from_rgba(235, 107, 52, 255), false);
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

    fn render_dt(&mut self) {
        const DT_SCALE: f32 = 1000.0;
        const MARGIN: f32 = 10.0;
        const PIXEL_SCALE: f32 = 2.0;
        const MAX_DTS: usize = 200;

        let dt = self.world.resource::<DT>().0 * DT_SCALE;
        self.dt_history.push(dt);
        if self.dt_history.len() > MAX_DTS {
            self.dt_history.remove(0);
        }

        for (i, &d) in self.dt_history.iter().enumerate() {
            let x = MARGIN + i as f32 * PIXEL_SCALE;
            let y = screen_height() - MARGIN - d * PIXEL_SCALE;
            let color = if d < 16.0 {
                // > 60 FPS
                GREEN
            } else if d < 33.0 {
                // > 30 FPS
                YELLOW
            } else {
                // < 30 FPS
                RED
            };
            draw_rectangle(x, y, PIXEL_SCALE, d * PIXEL_SCALE, color);
        }
    }
}
