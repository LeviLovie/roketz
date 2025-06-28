mod circle;

use anyhow::{Context, Result};
use macroquad::prelude::*;

use crate::AABB;

pub mod types {
    use super::*;

    pub use circle::Circle;
}

pub trait ColliderTrait {
    fn aabb(&self) -> AABB;
    fn collides_with(&self, other: &Collider) -> bool;
    fn sweep_test(&self, other: &Collider, delta: Vec2) -> Option<Vec2>;
}

pub struct Collider {
    pub ty: Box<dyn ColliderTrait>,
    pub rotation: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub changed: bool,
}

impl Collider {
    pub fn new<T>(ty: T) -> Result<Self>
    where
        T: TryInto<Box<dyn ColliderTrait>>,
        T::Error: std::error::Error + Send + Sync + 'static,
    {
        let ty = ty
            .try_into()
            .context("Failed to convert into ColliderTrait")?;

        Ok(Self {
            ty,
            rotation: 0.0,
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            changed: true,
        })
    }

    pub fn aabb(&self) -> AABB {
        self.ty.aabb().translate(self.position)
    }

    pub fn collides_with(&self, other: &Collider) -> bool {
        self.ty.collides_with(other)
    }

    pub fn sweep_test(&self, other: &Collider, delta: Vec2) -> Option<Vec2> {
        self.ty.sweep_test(other, delta)
    }
}
