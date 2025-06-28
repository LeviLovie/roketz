//! Collider module for Bonk2D

mod circle;

use anyhow::Result;
use macroquad::prelude::*;

use crate::AABB;

/// Different colliders implemented in Bonk2D.
pub mod types {
    use super::*;

    pub use circle::Circle;
}

/// Universial collider trait that all colliders must implement.
pub trait ColliderTrait {
    fn aabb(&self) -> Result<AABB>;
    fn collides(&self, other: &Collider) -> Result<bool>;
    fn sweep(&self, other: &Collider, delta: Vec2) -> Result<f32>;
}

pub enum Collider {
    Circle(types::Circle),
}

impl Collider {
    pub fn aabb(&self) -> Result<AABB> {
        match self {
            Collider::Circle(circle) => circle.aabb(),
        }
    }

    pub fn collides(&self, other: &Collider) -> Result<bool> {
        match self {
            Collider::Circle(circle) => circle.collides(other),
        }
    }

    pub fn sweep(&self, other: &Collider, delta: Vec2) -> Result<f32> {
        match self {
            Collider::Circle(circle) => circle.sweep(other, delta),
        }
    }
}

impl std::fmt::Debug for Collider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Collider::Circle(circle) => write!(f, "Circle({:?})", circle),
        }
    }
}
