use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, Query},
};
use nalgebra::{Vector2, Vector3};

use crate::ecs::{Texture, Transform};

#[derive(Component)]
pub struct Rocket {
    vel: Vector2<f32>,
    lifetime: f32,
}

fn spawn_rocket(commands: &mut Commands) {
    let spawn_distance = 600.0;
    let spawn_angle = rand::random::<f32>() * std::f32::consts::TAU;
    let position = Vector3::new(
        spawn_angle.cos() * spawn_distance,
        spawn_angle.sin() * spawn_distance,
        0.0,
    );
    let angle = spawn_angle + std::f32::consts::PI + (rand::random::<f32>() - 0.5);
    let speed = 2.0 + rand::random::<f32>() * 2.0;
    let vel = Vector2::new(angle.cos() * speed, angle.sin() * speed);
    let lifetime = 10.0 + rand::random::<f32>() * 5.0;
    commands.spawn((
        Rocket { vel, lifetime },
        Transform {
            position,
            scale: Vector2::new(1.0, 1.0),
            rotation: angle.to_degrees(),
            layer: 1,
        },
        Texture {
            handle: None,
            rotation: 90.0,
            path: "rocket_moving.png".to_string(),
        },
    ));
}

pub fn spawn_rockets(mut commands: Commands) {
    if rand::random::<f32>() < 0.01 {
        spawn_rocket(&mut commands);
    }
}

pub fn update_rockets(
    mut commands: Commands,
    mut rockets: Query<(Entity, &mut Rocket, &mut Transform)>,
) {
    for (entity, mut rocket, mut transform) in rockets.iter_mut() {
        transform.position.x += rocket.vel.x;
        transform.position.y += rocket.vel.y;
        rocket.lifetime -= 1.0 / 60.0;
        if rocket.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
