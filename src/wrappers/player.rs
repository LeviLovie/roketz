use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::game::GameData;

pub struct Player {
    data: Arc<Mutex<GameData>>,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    rotation: f32,
    pub speed: f32,
    pub drag: f32,
}

impl Player {
    pub fn new(data: Arc<Mutex<GameData>>) -> Self {
        Self {
            data,
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            rotation: 0.0,
            speed: 150.0,
            drag: 0.9975,
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

        if is_key_down(KeyCode::A) {
            self.rotation -= 0.1;
        } else if is_key_down(KeyCode::D) {
            self.rotation += 0.1;
        }
        if is_key_down(KeyCode::W) {
            self.acceleration += Vec2::new(self.rotation.cos(), self.rotation.sin()) * self.speed;
        }

        self.velocity += self.acceleration * get_frame_time();
        self.position += self.velocity * get_frame_time();

        if self.velocity.length() < 1.0 {
            self.velocity = Vec2::ZERO;
        } else {
            self.velocity = Vec2::new(self.velocity.x * self.drag, self.velocity.y * self.drag);
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
