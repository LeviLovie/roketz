//! Collider module for Bonk2D

mod circle;

use anyhow::Result;
use macroquad::prelude::*;

use crate::AABB;

/// Universial collider trait that all colliders must implement.
pub trait Collider {
    fn aabb(&self) -> Result<AABB>;
    fn collides(&self, other: &dyn Collider) -> Result<bool>;
    fn sweep(&self, other: &dyn Collider, delta: Vec2) -> Result<f32>;
}

/// Different colliders implemented in Bonk2D.
pub mod types {
    use super::*;

    /// Circle collider
    pub use circle::Circle;
}
