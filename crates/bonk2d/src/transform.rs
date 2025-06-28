//! Transform component for 2D entities.
//! This component holds position, rotation, and velocity data,

use bevy_ecs::prelude::*;
use macroquad::prelude::*;

/// Transform component for 2D entities.
#[derive(Component, Debug)]
pub struct Transform {
    pub pos: Vec2,
    pub rot: f32,
    pub vel: Vec2,
    pub changed: bool,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            pos: Vec2::ZERO,
            rot: 0.0,
            vel: Vec2::ZERO,
            changed: false,
        }
    }
}

impl Transform {
    /// Returns a reference to position
    pub fn pos(&self) -> &Vec2 {
        &self.pos
    }

    /// Returns a mutable reference to position
    pub fn pos_mut(&mut self) -> &mut Vec2 {
        &mut self.pos
    }

    /// Creates a new Transform with the given position, rotation, and velocity.
    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
        self.changed = true;
    }

    /// Returns a reference to rotation
    pub fn rot(&self) -> &f32 {
        &self.rot
    }

    /// Returns a mutable reference to rotation
    pub fn rot_mut(&mut self) -> &mut f32 {
        &mut self.rot
    }

    /// Sets the rotation of the transform.
    pub fn set_rot(&mut self, rot: f32) {
        self.rot = rot;
        self.changed = true;
    }

    /// Returns a reference to velocity
    pub fn vel(&self) -> &Vec2 {
        &self.vel
    }

    /// Returns a mutable reference to velocity
    pub fn vel_mut(&mut self) -> &mut Vec2 {
        &mut self.vel
    }

    /// Sets the velocity of the transform.
    pub fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel;
        self.changed = true;
    }

    /// Applies a force to the velocity.
    pub fn apply_vel(&mut self, delta: f32) {
        self.pos += self.vel * delta;
        self.changed = true;
    }

    /// Rotates the transform by the given delta angle.
    pub fn rotate(&mut self, delta: f32) {
        self.rot += delta;
        self.changed = true;
    }

    /// Checks if the transform has changed since the last update.
    pub fn is_changed(&self) -> bool {
        self.changed
    }
}
