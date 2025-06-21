use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use super::Terrain;
use crate::{
    bvh::{BVHNode, AABB},
    game::GameData,
};

pub struct Player {
    data: Arc<Mutex<GameData>>,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    rotation: f32,
    last_position: Vec2,
    // ship params
    pub rotation_speed: f32,
    pub thrust: f32,
    pub drag: f32,
    pub weight: f32,
    // environment params
    pub gravity: f32,
    spawn_point: Vec2,
    // debug data
    position_before_collision: Vec2,
    // collisions
    collider_radius: f32,
    nearby_nodes: Vec<(BVHNode, AABB)>,
}

impl Player {
    pub fn new(data: Arc<Mutex<GameData>>, spawn_point: Vec2) -> Self {
        let mut player = Self {
            data,
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            rotation: 0.0,
            last_position: Vec2::ZERO,
            rotation_speed: 5.0,
            thrust: 150.0,
            drag: 0.9975,
            weight: 1.0,
            gravity: 9.81,
            spawn_point,
            position_before_collision: Vec2::ZERO,
            collider_radius: 3.0,
            nearby_nodes: Vec::new(),
        };

        player.respawn();

        player
    }

    pub fn respawn(&mut self) {
        self.position = self.spawn_point;
        self.velocity = Vec2::ZERO;
        self.acceleration = Vec2::ZERO;
        self.rotation = std::f32::consts::PI / -2.0;
        self.last_position = self.spawn_point;
        self.position_before_collision = self.spawn_point;
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
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.rotation -= self.rotation_speed * dt;
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.rotation += self.rotation_speed * dt;
        }

        // --- Thrust force ---
        let thrust_force = match is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            true => {
                let direction = Vec2::new(self.rotation.cos(), self.rotation.sin());
                direction * self.thrust
            }
            false => Vec2::ZERO,
        };

        // --- Gravity force (F = m * g) ---
        let gravity_force = Vec2::new(0.0, self.gravity * self.weight);

        // --- Combine forces & calculate acceleration (F = m * a => a = F / m) ---
        let net_force = thrust_force + gravity_force;
        self.acceleration = net_force / self.weight;

        // Update velocity and position
        self.velocity *= self.drag;
        self.velocity += self.acceleration * dt;

        self.last_position = self.position;
        self.position += self.velocity * dt;

        self.position_before_collision = self.position;
        self.collide_with_terrain(terrain);

        // if self.position.y >= max_height {
        //     self.position.y = max_height;
        //     // Reset when hitting the ground
        //     self.acceleration = Vec2::ZERO;
        //     self.velocity = Vec2::ZERO;
        //     self.rotation = 3.0 * std::f32::consts::PI / 2.0;
        // }

        if self.velocity.length() < 0.1 {
            self.velocity = Vec2::ZERO;
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
        draw_circle(
            self.position.x,
            self.position.y,
            self.collider_radius,
            WHITE,
        );
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.rotation.cos() * self.collider_radius * 1.5,
            self.position.y + self.rotation.sin() * self.collider_radius * 1.5,
            2.0,
            WHITE,
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

            for (node, bounds) in &self.nearby_nodes {
                node.draw(*bounds, 0, 10);
            }
        }
    }

    pub fn ui(&self, ctx: &egui::Context) {
        if self.data.lock().unwrap().debug.v_player {
            egui::Window::new("Player")
                .default_width(300.0)
                .show(ctx, |ui| {
                    ui.label(format!(
                        "Position: ({:.2}, {:.2})",
                        self.position.x, self.position.y
                    ));
                    ui.label(format!(
                        "Velocity: ({:.2}, {:.2}) ({:.2})",
                        self.velocity.x,
                        self.velocity.y,
                        self.velocity.length()
                    ));
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
                    ui.label(format!("Nearby Nodes: {}", self.nearby_nodes.len()));
                });
        }
    }
}
