use anyhow::{Context, Result};
use macroquad::prelude::*;

use super::{Collider, ColliderTrait};
use crate::{Transform, AABB};

#[derive(Debug)]
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

impl ColliderTrait for Circle {
    fn aabb(&self) -> Result<AABB> {
        let aabb = AABB::from_center_size(
            *self.transform.pos(),
            vec2(self.radius * 2.0, self.radius * 2.0),
        )
        .context("Failed to create AABB from Circle collider")?;
        Ok(aabb.translate(*self.transform.pos()))
    }

    fn collides(&self, other: &Collider) -> Result<bool> {
        // Skip checks if AABBs dont intersect
        if !self.aabb()?.intersects(&other.aabb()?) {
            return Ok(false);
        }

        // Match the type of the other collider
        match other {
            Collider::Circle(other) => {
                let distance = self
                    .transform
                    .pos()
                    .distance_squared(*other.transform.pos());
                let combined_radius = self.radius + other.radius;
                Ok(distance <= combined_radius * combined_radius)
            }
        }
    }

    fn sweep(&self, other: &Collider, _delta: Vec2) -> Result<f32> {
        // Skip checks if AABBs dont intersect
        if !self.aabb()?.intersects(&other.aabb()?) {
            return Ok(1.0);
        }

        match other {
            Collider::Circle(_other) => {
                unimplemented!("Sweep test for Circle vs Circle is not implemented yet");
            }
        }
    }
}
