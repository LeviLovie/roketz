use bevy_ecs::{component::Component, system::Commands};
use nalgebra::{Vector2, Vector3};

use crate::ecs::{Texture, Transform};

#[derive(Component)]
pub struct Background {}

pub fn spawn_background(mut commands: Commands) {
    commands.spawn((
        Background {},
        Transform {
            position: Vector3::new(384.0, 384.0, 0.0),
            scale: Vector2::new(3.0, 3.0),
            rotation: 0.0,
            layer: 0,
        },
        Texture {
            handle: None,
            path: "background.png".to_string(),
        },
    ));
}
