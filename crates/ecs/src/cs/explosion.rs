use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use tracing::{debug, warn};

use crate::cs::{Terrain, TerrainCollider};

#[derive(Component, Debug, Clone)]
pub struct Explosion {
    pub position: Vec2,
    pub radius: f32,
    pub damage: f32,
}

impl Explosion {
    pub fn new(position: Vec2, radius: f32, damage: f32) -> Self {
        Explosion {
            position,
            radius,
            damage,
        }
    }
}

pub fn update_explosions(
    mut commands: Commands,
    explosions: Query<(Entity, &Explosion)>,
    terrain: Query<&Terrain>,
    colliders: Query<(Entity, &TerrainCollider)>,
) {
    for (entity, explosion) in explosions.iter() {
        debug!(
            "Explosion at {:?} with radius {} and damage {}",
            explosion.position, explosion.radius, explosion.damage
        );
        if let Ok(terrain) = terrain.single() {
            for collider in terrain
                .bvh
                .find_intersects_circle(explosion.position, explosion.radius)
            {
                debug!(
                    "Checking collider {:?} for explosion at {:?}",
                    collider, explosion.position
                );
            }
        } else {
            warn!("No terrain found for explosion at {:?}", explosion.position);
        }
        commands.entity(entity).despawn();
    }
}
