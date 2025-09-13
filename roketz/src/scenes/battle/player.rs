use bevy_ecs::{
    component::Component,
    query::With,
    system::{Commands, Query, Res, ResMut},
};
use nalgebra::{Vector2, Vector3};
use rapier2d::prelude::ColliderBuilder;
use winit::keyboard::{Key, SmolStr};

use crate::ecs::*;

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(mut commands: Commands, rapier: ResMut<RapierWorld>) {
    let collider = ColliderBuilder::cuboid(16.0, 16.0).build();
    let rigidbody = RigidBody::dynamic(
        rapier,
        collider,
        nalgebra::Vector2::new(0.0, 0.0),
        180.0_f32.to_radians(),
    );
    commands.spawn((
        Player {},
        rigidbody,
        Transform {
            position: Vector3::new(0.0, 0.0, 0.2),
            scale: Vector2::new(3.0, 3.0),
            rotation: 0.0,
            layer: 2,
        },
        Texture {
            handle: None,
            rotation: 90.0,
            path: "rocket_moving.png".to_string(),
        },
        CameraTarget {},
    ));
}

pub fn update_players(
    mut players: Query<(&Transform, &mut RigidBody), With<Player>>,
    inputs: Res<Inputs>,
    mut rapier: ResMut<RapierWorld>,
) {
    let inputs = inputs.0.lock_panic();
    for (transform, rigidbody) in players.iter_mut() {
        if let Some(body) = rapier.bodies.get_mut(rigidbody.body) {
            let rotation_radians = transform.rotation.to_radians();
            let forward = Vector2::new(rotation_radians.cos(), rotation_radians.sin());

            let mut linvel = *body.linvel();
            if inputs.is_key_down(Key::Character(SmolStr::from("w"))) {
                linvel += forward * 2.0;
            } else if inputs.is_key_down(Key::Character(SmolStr::from("s"))) {
                linvel -= forward * 2.0;
            } else {
                linvel *= 0.95;
                if linvel.magnitude() < 0.1 {
                    linvel = Vector2::new(0.0, 0.0);
                }
            }
            body.set_linvel(linvel, true);

            let mut angvel = body.angvel();
            if inputs.is_key_down(Key::Character(SmolStr::from("a"))) {
                angvel += 0.1;
            } else if inputs.is_key_down(Key::Character(SmolStr::from("d"))) {
                angvel -= 0.1;
            } else {
                angvel *= 0.9;
                if angvel.abs() < 0.01 {
                    angvel = 0.0;
                }
            }
            body.set_angvel(angvel, true);
        }
    }
}
