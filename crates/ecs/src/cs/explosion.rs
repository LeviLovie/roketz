use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use tracing::warn;

use crate::cs::Terrain;

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
    mut terrain: Query<&mut Terrain>,
) {
    for (entity, explosion) in explosions.iter() {
        if let Ok(mut terrain) = terrain.single_mut() {
            if let Err(e) = terrain.destruct(
                explosion.position.x as u32,
                explosion.position.y as u32,
                explosion.radius as u32,
            ) {
                error!("Failed to destruct terrain: {}", e);
            }
        } else {
            warn!("No terrain found for explosion at {:?}", explosion.position);
        }
        commands.entity(entity).despawn();
    }
}
