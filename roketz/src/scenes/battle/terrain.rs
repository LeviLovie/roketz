use bevy_ecs::{component::Component, system::Commands};
use nalgebra::Vector2;

use crate::ecs::{Texture, Transform};

#[derive(Component)]
pub struct Terrain {}

pub fn spawn_terrain(mut commands: Commands) {
    commands.spawn((
        Transform {
            position: Vector2::new(100.0, 100.0),
            scale: Vector2::new(3.0, 3.0),
            rotation: 180.0,
            layer: 1,
        },
        Texture {
            handle: None,
            path: "map.png".to_string(),
            width: 256,
            height: 256,
        },
    ));
}
