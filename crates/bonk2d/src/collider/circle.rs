use anyhow::{Context, Result, bail};
use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use super::{Collider, ColliderTrait};
use crate::{AABB, Transform};

#[derive(Component, Debug)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Circle { radius }
    }
}

impl ColliderTrait for Circle {
    fn aabb(&self, transform: &Transform) -> Result<AABB> {
        let aabb =
            AABB::from_center_size(*transform.pos(), vec2(self.radius * 2.0, self.radius * 2.0))
                .context("Failed to create AABB from Circle collider")?;
        Ok(aabb)
    }

    fn collides(
        &self,
        transform: &Transform,
        other: &Collider,
        other_transform: &Transform,
    ) -> Result<bool> {
        // Skip checks if AABBs dont intersect
        if !self
            .aabb(transform)?
            .intersects(&other.aabb(other_transform)?)
        {
            return Ok(false);
        }

        // Match the type of the other collider
        match other {
            Collider::Circle(other) => {
                let distance = transform.pos().distance_squared(*other_transform.pos());
                let combined_radius = self.radius + other.radius;
                Ok(distance <= combined_radius * combined_radius)
            }
        }
    }

    fn sweep(
        &self,
        transform: &Transform,
        other: &Collider,
        other_transform: &Transform,
        _delta: Vec2,
    ) -> Result<f32> {
        // Skip checks if AABBs dont intersect
        if !self
            .aabb(transform)?
            .intersects(&other.aabb(other_transform)?)
        {
            return Ok(1.0);
        }

        match other {
            Collider::Circle(_other) => {
                bail!("Sweep test not implemented for Circle colliders");
            }
        }
    }
}
