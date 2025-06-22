use egui::{Checkbox, CollapsingHeader, ComboBox, DragValue, Grid};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use super::{Bullet, BulletType, Terrain};
use crate::{
    bvh::{BVHNode, AABB},
    game::GameData,
};

pub struct PlayerBuilder {
    player: Player,
}

impl PlayerBuilder {
    pub fn new(data: Arc<Mutex<GameData>>) -> Self {
        Self {
            player: Player {
                data,
                health: 100.0,
                position: Vec2::ZERO,
                velocity: Vec2::ZERO,
                acceleration: Vec2::ZERO,
                rotation: 0.0,
                last_position: Vec2::ZERO,
                thrust_force: Vec2::ZERO,
                is_player_2: false,
                is_dead: false,
                respawn_timer: 0.0,
                rotation_speed: 5.0,
                thrust: 150.0,
                drag: 0.9975,
                weight: 1.0,
                bullet_type: BulletType::Simple,
                bullet_cooldown: 0.0,
                gravity: 9.81,
                spawn_point: Vec2::ZERO,
                kill_distance_x: 1000.0,
                kill_distance_y: 1000.0,
                kill_point: Vec2::ZERO,
                position_before_collision: Vec2::ZERO,
                collider_radius: 3.0,
                nearby_nodes: Vec::new(),
                no_collision: false,
            },
        }
    }

    pub fn with_spawn_point(mut self, spawn_point: Vec2) -> Self {
        self.player.spawn_point = spawn_point;
        self
    }

    pub fn with_gravity(mut self, gravity: f32) -> Self {
        self.player.gravity = gravity;
        self
    }

    pub fn with_terrain_data(mut self, terrain: &Terrain) -> Self {
        self.player.kill_distance_x = terrain.kill_distance_x as f32;
        self.player.kill_distance_y = terrain.kill_distance_y as f32;
        self.player.kill_point = vec2(terrain.width as f32 / 2.0, terrain.height as f32 / 2.0);
        self
    }

    pub fn is_player_2(mut self, is_player_2: bool) -> Self {
        self.player.is_player_2 = is_player_2;
        self
    }

    pub fn build(self) -> Player {
        let mut player = self.player;
        player.respawn();
        player
    }
}

pub struct Player {
    data: Arc<Mutex<GameData>>,
    health: f32,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    rotation: f32,
    last_position: Vec2,
    thrust_force: Vec2,
    is_player_2: bool,
    is_dead: bool,
    respawn_timer: f32,
    // ship params
    pub rotation_speed: f32,
    pub thrust: f32,
    pub drag: f32,
    pub weight: f32,
    bullet_type: BulletType,
    bullet_cooldown: f32,
    // environment params
    pub gravity: f32,
    spawn_point: Vec2,
    kill_distance_x: f32,
    kill_distance_y: f32,
    kill_point: Vec2,
    // debug data
    position_before_collision: Vec2,
    // collisions
    collider_radius: f32,
    nearby_nodes: Vec<(BVHNode, AABB)>,
    no_collision: bool,
}

impl Player {
    pub fn builder(data: Arc<Mutex<GameData>>) -> PlayerBuilder {
        PlayerBuilder::new(data)
    }

    pub fn kill(&mut self) {
        self.is_dead = true;
        self.health = 0.0;
        self.respawn_timer = 5.0;
    }

    pub fn respawn(&mut self) {
        self.respawn_timer = 0.0;
        self.position = self.spawn_point;
        self.velocity = Vec2::ZERO;
        self.acceleration = Vec2::ZERO;
        self.rotation = std::f32::consts::PI / -2.0;
        self.last_position = self.spawn_point;
        self.is_dead = false;
        self.health = 100.0;
        self.position_before_collision = self.spawn_point;
        self.nearby_nodes.clear();
    }

    pub fn teleport(&mut self, position: Vec2, rotation: f32) {
        self.position = position;
        self.rotation = rotation;
        self.velocity = Vec2::ZERO;
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn update(&mut self, terrain: &mut Terrain, bullets: &mut Vec<Bullet>) {
        let dt = get_frame_time();

        if !self.is_dead && self.health <= 0.0 {
            self.kill();
        }

        if self.is_dead {
            if self.respawn_timer < dt {
                self.respawn_timer = 0.0;
                self.respawn();
            } else {
                self.respawn_timer -= dt;
            }

            return;
        }

        self.acceleration = Vec2::ZERO;

        // --- Rotation input ---
        if !self.is_player_2 && is_key_down(KeyCode::A)
            || (self.is_player_2 && is_key_down(KeyCode::J))
        {
            self.rotation -= self.rotation_speed * dt;
        }
        if !self.is_player_2 && is_key_down(KeyCode::D)
            || (self.is_player_2 && is_key_down(KeyCode::L))
        {
            self.rotation += self.rotation_speed * dt;
        }

        // --- Thrust force ---
        if !self.is_player_2 && is_key_down(KeyCode::W)
            || self.is_player_2 && is_key_down(KeyCode::I)
        {
            let direction = Vec2::new(self.rotation.cos(), self.rotation.sin());
            self.thrust_force = direction * self.thrust;
        } else {
            self.thrust_force = Vec2::ZERO;
        };

        // --- Gravity force (F = m * g) ---
        let gravity_force = Vec2::new(0.0, self.gravity * self.weight);

        // --- Combine forces & calculate acceleration (F = m * a => a = F / m) ---
        let net_force = self.thrust_force + gravity_force;
        self.acceleration = net_force / self.weight;

        // Update velocity and position
        self.velocity *= self.drag;
        self.velocity += self.acceleration * dt;

        self.last_position = self.position;
        self.position += self.velocity * dt;

        self.position_before_collision = self.position;
        if !self.no_collision {
            self.collide_with_terrain(terrain);
        }

        if self.velocity.length() < 1.0 {
            self.velocity = Vec2::ZERO;
        }

        if self.position.x < self.kill_point.x - self.kill_distance_x
            || self.position.x > self.kill_point.x + self.kill_distance_x
            || self.position.y < self.kill_point.y - self.kill_distance_y
            || self.position.y > self.kill_point.y + self.kill_distance_y
        {
            self.respawn();
        }

        if self.bullet_cooldown > dt {
            self.bullet_cooldown -= dt;
        } else {
            self.bullet_cooldown = 0.0;
        }

        if (self.is_player_2 && is_key_down(KeyCode::Semicolon)
            || !self.is_player_2 && is_key_down(KeyCode::Space))
            && self.bullet_cooldown <= 0.0
        {
            self.bullet_cooldown = self.bullet_type.cooldown();
            bullets.push(Bullet::new(
                self.data.clone(),
                self.position
                    + Vec2::new(
                        self.rotation.cos() * self.collider_radius * 1.5,
                        self.rotation.sin() * self.collider_radius * 1.5,
                    ),
                self.rotation,
                self.bullet_type,
            ));
        }

        for bullet in bullets.iter_mut() {
            if bullet.is_alive() {
                self.collide_with_bullet(bullet);
            }
        }
    }

    fn collide_with_bullet(&mut self, bullet: &mut Bullet) {
        if bullet.position().distance(self.position) < self.collider_radius {
            self.health -= bullet.ty.damage();
            bullet.kill();

            return;
        }
    }

    pub fn collide_with_terrain(&mut self, terrain: &mut Terrain) {
        let nearby_radius = self
            .data
            .lock()
            .unwrap()
            .config
            .physics
            .collisions
            .nearby_nodes_radius;
        let nearby_nodes = terrain.bvh.get_nearby_nodes(self.position, nearby_radius);

        for (_node, bounds) in &nearby_nodes {
            if bounds.intersects_circle(self.position, self.collider_radius) {
                let distance = self.position.distance(bounds.center());
                if distance < self.collider_radius {
                    let overlap = self.collider_radius - distance;
                    let normal = (self.position - bounds.center()).normalize();
                    self.position += normal * overlap;

                    let max_crash_velocity =
                        self.data.lock().unwrap().config.physics.max_crash_velocity;
                    if self.velocity.length() > max_crash_velocity {
                        terrain.destruct(self.position.x as u32, self.position.y as u32, 20);
                        self.kill();
                    }

                    self.velocity = Vec2::ZERO;
                }
            }
        }

        self.nearby_nodes = nearby_nodes;
    }

    pub fn draw(&self) {
        if self.is_dead {
            return;
        }

        let color = if self.is_player_2 {
            Color::from_rgba(245, 122, 56, 255)
        } else {
            Color::from_rgba(66, 245, 230, 255)
        };

        draw_circle(
            self.position.x,
            self.position.y,
            self.collider_radius,
            color,
        );
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.rotation.cos() * self.collider_radius * 1.5,
            self.position.y + self.rotation.sin() * self.collider_radius * 1.5,
            2.0,
            color,
        );

        let health_bar_full_length = 20.0;
        let health_bar_length = health_bar_full_length / 100.0 * self.health;
        let health_bar_color = if self.health > 50.0 {
            Color::from_rgba(0, 255, 0, 100)
        } else if self.health > 20.0 {
            Color::from_rgba(255, 255, 0, 100)
        } else {
            Color::from_rgba(255, 0, 0, 100)
        };
        draw_rectangle(
            self.position.x - health_bar_full_length / 2.0,
            self.position.y - 6.0,
            health_bar_length,
            1.0,
            health_bar_color,
        );
        draw_rectangle(
            self.position.x - health_bar_full_length / 2.0 + health_bar_length,
            self.position.y - 6.0,
            health_bar_full_length - health_bar_length,
            1.0,
            Color::from_rgba(0, 0, 0, 100),
        );

        if self.data.lock().unwrap().debug.ol_physics {
            draw_line(
                self.position.x,
                self.position.y,
                self.position.x + self.velocity.x,
                self.position.y + self.velocity.y,
                1.0,
                RED,
            );
            draw_line(
                self.position.x,
                self.position.y,
                self.position.x + self.acceleration.x,
                self.position.y + self.acceleration.y,
                1.0,
                BLUE,
            );

            draw_circle_lines(
                self.position_before_collision.x,
                self.position_before_collision.y,
                self.collider_radius,
                0.5,
                RED,
            );
            draw_circle_lines(
                self.position.x,
                self.position.y,
                self.collider_radius,
                0.5,
                GREEN,
            );

            draw_rectangle_lines(
                self.kill_point.x - self.kill_distance_x,
                self.kill_point.y - self.kill_distance_y,
                self.kill_distance_x * 2.0,
                self.kill_distance_y * 2.0,
                1.0,
                Color::from_rgba(255, 0, 0, 100),
            );

            for (node, bounds) in &self.nearby_nodes {
                node.draw(*bounds, 0, 10);
            }
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if self.data.lock().unwrap().debug.v_player {
            let window_title = if self.is_player_2 {
                "Player 2"
            } else {
                "Player 1"
            };

            egui::Window::new(window_title)
                .default_width(300.0)
                .show(ctx, |ui| {
                    CollapsingHeader::new("Transform")
                        .default_open(true)
                        .show(ui, |ui| {
                            Grid::new(format!("transform_{}", self.is_player_2))
                                .num_columns(4)
                                .show(ui, |ui| {
                                    let available_width = ui.available_width();
                                    ui.label("Position:");
                                    ui.add_sized(
                                        [available_width, 20.0],
                                        DragValue::new(&mut self.position.x)
                                            .speed(0.1)
                                            .prefix("X "),
                                    );
                                    ui.add_sized(
                                        [available_width, 20.0],
                                        DragValue::new(&mut self.position.y)
                                            .speed(0.1)
                                            .prefix("Y "),
                                    );
                                    ui.end_row();
                                    ui.label("Velocity:");
                                    ui.add_sized(
                                        [available_width, 20.0],
                                        DragValue::new(&mut self.velocity.x)
                                            .speed(0.1)
                                            .prefix("X "),
                                    );
                                    ui.add_sized(
                                        [available_width, 20.0],
                                        DragValue::new(&mut self.velocity.y)
                                            .speed(0.1)
                                            .prefix("Y "),
                                    );
                                    ui.label(format!("L {:.2}", self.velocity.length()));
                                    ui.end_row();
                                    ui.label("Acceleration:");
                                    ui.add_sized(
                                        [available_width, 20.0],
                                        DragValue::new(&mut self.acceleration.x)
                                            .speed(0.1)
                                            .prefix("X "),
                                    );
                                    ui.add_sized(
                                        [available_width, 20.0],
                                        DragValue::new(&mut self.acceleration.y)
                                            .speed(0.1)
                                            .prefix("Y "),
                                    );
                                    ui.label(format!("L {:.2}", self.acceleration.length()));
                                    ui.end_row();
                                    ui.label("Rotation:");
                                    let mut degrees = self.rotation.to_degrees();
                                    ui.add_sized(
                                        [available_width, 20.0],
                                        DragValue::new(&mut degrees).speed(0.1).suffix("°"),
                                    );
                                    self.rotation = degrees.to_radians();
                                    ui.add_sized(
                                        [available_width, 20.0],
                                        DragValue::new(&mut self.rotation).speed(0.05).suffix("c"),
                                    );
                                    ui.end_row();
                                });
                        });

                    CollapsingHeader::new("Health")
                        .default_open(false)
                        .show(ui, |ui| {
                            Grid::new(format!("health_{}", self.is_player_2))
                                .num_columns(2)
                                .show(ui, |ui| {
                                    ui.label("Health:");
                                    ui.horizontal(|ui| {
                                        if ui.button("-25").clicked() {
                                            self.health -= 25.0;
                                        }
                                        if ui.button("-5").clicked() {
                                            self.health -= 5.0;
                                        }
                                        ui.add_enabled(
                                            self.health > 0.0,
                                            DragValue::new(&mut self.health),
                                        );
                                        if ui.button("+5").clicked() {
                                            self.health += 5.0;
                                        }
                                        if ui.button("+25").clicked() {
                                            self.health += 25.0;
                                        }
                                    });
                                    ui.end_row();
                                    ui.label("Is Dead:");
                                    ui.add_enabled(false, Checkbox::new(&mut self.is_dead, ""));
                                    ui.end_row();
                                    ui.label("Respawn:");
                                    ui.label(format!("{:.2}s", self.respawn_timer));
                                });

                            ui.horizontal(|ui| {
                                if ui.button("Kill").clicked() {
                                    self.kill();
                                }
                                if ui.button("Respawn").clicked() {
                                    self.respawn();
                                }
                                if ui.button("Heal").clicked() && !self.is_dead {
                                    self.health = 100.0;
                                }
                            });
                        });

                    CollapsingHeader::new("Collisions")
                        .default_open(false)
                        .show(ui, |ui| {
                            Grid::new(format!("collisions_{}", self.is_player_2))
                                .num_columns(2)
                                .show(ui, |ui| {
                                    ui.label("Nodes");
                                    ui.label(format!("{}", self.nearby_nodes.len()));
                                    ui.end_row();
                                    ui.label("No collisions");
                                    ui.checkbox(&mut self.no_collision, "");
                                });
                        });

                    CollapsingHeader::new("Bullets")
                        .default_open(false)
                        .show(ui, |ui| {
                            Grid::new(format!("collisions_{}", self.is_player_2))
                                .num_columns(2)
                                .show(ui, |ui| {
                                    ui.label("Type");
                                    ComboBox::from_label("")
                                        .selected_text(self.bullet_type.to_string())
                                        .show_ui(ui, |ui| {
                                            for variant in BulletType::variants() {
                                                ui.selectable_value(
                                                    &mut self.bullet_type,
                                                    variant,
                                                    variant.to_string(),
                                                );
                                            }
                                        });
                                    ui.end_row();
                                    ui.label("Cooldown");
                                    ui.label(format!("{:.2}s", self.bullet_cooldown));
                                });
                        });
                });
        }
    }
}
