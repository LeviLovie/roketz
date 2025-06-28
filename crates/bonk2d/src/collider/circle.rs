use anyhow::{Context, Result};
use macroquad::prelude::*;

use super::Collider;
use crate::{Transform, AABB};

pub struct Circle {
    pub radius: f32,
    pub transform: Transform,
}

impl Circle {
    pub fn new<T>(transform: T, radius: f32) -> Self
    where
        T: Into<Transform>,
    {
        Circle {
            radius,
            transform: transform.into(),
        }
    }
}

impl Collider for Circle {
    fn aabb(&self) -> Result<AABB> {
        AABB::from_center_size(
            *self.transform.pos(),
            vec2(self.radius * 2.0, self.radius * 2.0),
        )
        .context("Failed to create AABB from Circle collider")
    }

    fn collides(&self, other: &dyn Collider) -> Result<bool> {
        // Skip checks if AABBs dont intersect
        if !self.aabb()?.intersects(&other.aabb()?) {
            return Ok(false);
        }

        // Match the type of the other collider
        Ok(false)
    }

    fn sweep(&self, other: &dyn Collider, delta: Vec2) -> Result<f32> {
        // Skip checks if AABBs dont intersect
        if !self.aabb()?.translate(delta).intersects(&other.aabb()?) {
            return Ok(1.0);
        }

        unimplemented!("Sweep test for Circle collider is not implemented yet");
    }
}
