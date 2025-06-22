use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use super::Terrain;
use crate::{
    bvh::{AABB, BVHNode},
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
                position: Vec2::ZERO,
                velocity: Vec2::ZERO,
                acceleration: Vec2::ZERO,
                rotation: 0.0,
                last_position: Vec2::ZERO,
                thrust_force: Vec2::ZERO,
                is_player_2: false,
                rotation_speed: 5.0,
                thrust: 150.0,
                drag: 0.9975,
                weight: 1.0,
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
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    rotation: f32,
    last_position: Vec2,
    thrust_force: Vec2,
    is_player_2: bool,
    // ship params
    pub rotation_speed: f32,
    pub thrust: f32,
    pub drag: f32,
    pub weight: f32,
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

    pub fn respawn(&mut self) {
        self.position = self.spawn_point;
        self.velocity = Vec2::ZERO;
        self.acceleration = Vec2::ZERO;
        self.rotation = std::f32::consts::PI / -2.0;
        self.last_position = self.spawn_point;
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

    pub fn update(&mut self, terrain: &mut Terrain) {
        self.acceleration = Vec2::ZERO;

        let dt = get_frame_time();

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
                        self.respawn();
                    }

                    self.velocity = Vec2::ZERO;
                }
            }
        }

        self.nearby_nodes = nearby_nodes;
    }

    pub fn draw(&self) {
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
                    ui.label(format!(
                        "Position: ({:.2}, {:.2})",
                        self.position.x, self.position.y
                    ));
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "Velocity: ({:.2}, {:.2}) ({:.2})",
                            self.velocity.x,
                            self.velocity.y,
                            self.velocity.length()
                        ));
                        if ui.button("Reset").clicked() {
                            self.velocity = Vec2::ZERO;
                        }
                    });
                    ui.label(format!(
                        "Acceleration: ({:.2}, {:.2}) ({:.2})",
                        self.acceleration.x,
                        self.acceleration.y,
                        self.acceleration.length()
                    ));
                    ui.label(format!(
                        "Rotation: {:.2} rad {:.2} deg",
                        self.rotation,
                        self.rotation.to_degrees()
                    ));
                    if ui.button("Respawn").clicked() {
                        self.respawn();
                    }
                    ui.separator();
                    ui.label(format!("Nearby Nodes: {}", self.nearby_nodes.len()));
                    ui.checkbox(&mut self.no_collision, "No Collision");
                });
        }
    }
}
