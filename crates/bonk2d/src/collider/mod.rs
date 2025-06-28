//! Collider module for Bonk2D

mod circle;

use anyhow::Result;
use macroquad::prelude::*;

use crate::{AABB, Transform};

/// Different colliders implemented in Bonk2D.
pub mod types {
    use super::*;

    pub use circle::Circle;
}

/// Universial collider trait that all colliders must implement.
pub trait ColliderTrait {
    fn aabb(&self, transform: &Transform) -> Result<AABB>;
    fn collides(
        &self,
        transform: &Transform,
        other: &Collider,
        other_transform: &Transform,
    ) -> Result<bool>;
    fn sweep(
        &self,
        transform: &Transform,
        other: &Collider,
        other_transform: &Transform,
        delta: Vec2,
    ) -> Result<f32>;
}

pub enum Collider {
    Circle(types::Circle),
}

impl Collider {
    pub fn aabb(&self, transform: &Transform) -> Result<AABB> {
        match self {
            Collider::Circle(circle) => circle.aabb(transform),
        }
    }

    pub fn collides(
        &self,
        transform: &Transform,
        other: &Collider,
        other_transform: &Transform,
    ) -> Result<bool> {
        match self {
            Collider::Circle(circle) => circle.collides(transform, other, other_transform),
        }
    }

    pub fn sweep(
        &self,
        transform: &Transform,
        other: &Collider,
        other_transform: &Transform,
        delta: Vec2,
    ) -> Result<f32> {
        match self {
            Collider::Circle(circle) => circle.sweep(transform, other, other_transform, delta),
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
