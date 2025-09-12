use bevy_ecs::{
    component::Component,
    query::With,
    system::{Commands, Query, Res},
};
use nalgebra::Vector2;
use winit::keyboard::{Key, SmolStr};

use crate::ecs::{CameraTarget, Inputs, Texture, Transform};

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player {},
        Transform {
            position: Vector2::new(100.0, 100.0),
            scale: Vector2::new(3.0, 3.0),
            rotation: 180.0,
            layer: 2,
        },
        Texture {
            handle: None,
            path: "rocket_moving.png".to_string(),
            width: 32,
            height: 32,
        },
        CameraTarget {},
    ));
}

pub fn update_players(mut players: Query<&mut Transform, With<Player>>, inputs: Res<Inputs>) {
    let inputs = inputs.0.lock_panic();
    for mut transform in players.iter_mut() {
        if inputs.is_key_down(Key::Character(SmolStr::from("w"))) {
            transform.position.y -= 2.0;
        }
        if inputs.is_key_down(Key::Character(SmolStr::from("s"))) {
            transform.position.y += 2.0;
        }
        if inputs.is_key_down(Key::Character(SmolStr::from("a"))) {
            transform.position.x -= 2.0;
        }
        if inputs.is_key_down(Key::Character(SmolStr::from("d"))) {
            transform.position.x += 2.0;
        }
    }
}
