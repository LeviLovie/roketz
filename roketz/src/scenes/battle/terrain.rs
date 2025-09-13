use bevy_ecs::{component::Component, system::Commands};
use nalgebra::{Vector2, Vector3};

use crate::ecs::{Texture, Transform};

#[derive(Component)]
pub struct Terrain {}

pub fn spawn_terrain(mut commands: Commands) {
    commands.spawn((
        Terrain {},
        Transform {
            position: Vector3::new(0.0, 0.0, 0.1),
            scale: Vector2::new(3.0, 3.0),
            rotation: 180.0,
            layer: 2,
        },
        Texture {
            handle: None,
            rotation: 0.0,
            path: "map.png".to_string(),
        },
    ));
}
