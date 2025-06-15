use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::game::GameData;

pub struct Player {
    data: Arc<Mutex<GameData>>,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    rotation: f32,
    // ship params
    pub rotation_speed: f32,
    pub thrust: f32,
    pub drag: f32,
    pub weight: f32,
    // environment params
    pub gravity: f32,
}

impl Player {
    pub fn new(data: Arc<Mutex<GameData>>) -> Self {
        Self {
            data,
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            rotation: 0.0,
            // ship params
            rotation_speed: 0.05,
            thrust: 150.0,
            drag: 0.9975,
            weight: 1.0,
            // environment params
            gravity: 9.81,
        }
    }

    pub fn teleport(&mut self, position: Vec2, rotation: f32) {
        self.position = position;
        self.rotation = rotation;
        self.velocity = Vec2::ZERO;
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn update(&mut self) {
        self.acceleration = Vec2::ZERO;

        let dt = get_frame_time();

        // --- Rotation input ---
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.rotation -= self.rotation_speed;
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.rotation += self.rotation_speed;
        }

        // --- Thrust force ---
        let mut thrust_force = Vec2::ZERO;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            let direction = Vec2::new(self.rotation.cos(), self.rotation.sin());
            thrust_force = direction * self.thrust;
        }

        // --- Gravity force ---
        let gravity_force = Vec2::new(0.0, self.gravity * self.weight); // F = m * g

        // --- Combine forces & calculate acceleration (F = m * a => a = F / m) ---
        let net_force = thrust_force + gravity_force;
        self.acceleration = net_force / self.weight;

        // Update velocity and position
        self.velocity *= self.drag;
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;

        // Clamp position to screen bounds
        let max_width = screen_width() / 2.5;
        let max_height = screen_height() / 2.5;
        if self.position.x < -max_width {
            self.position.x = -max_width;
        } else if self.position.x > max_width {
            self.position.x = max_width;
        }
        if self.position.y <= -max_height {
            self.position.y = -max_height;
        } else if self.position.y >= max_height {
            self.position.y = max_height;
            // Reset when hitting the ground
            self.acceleration = Vec2::ZERO;
            self.velocity = Vec2::ZERO;
            self.rotation = 3.0 * 3.1416 / 2.0;
        }

        if self.velocity.length() < 0.01 {
            self.velocity = Vec2::ZERO;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 20.0, WHITE);
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.rotation.cos() * 30.0,
            self.position.y + self.rotation.sin() * 30.0,
            2.0,
            WHITE,
        );

        if self.data.lock().unwrap().is_debug {
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
        }
    }
}
