use bevy_ecs::{component::Component, system::ResMut};
use rapier2d::prelude::*;

use crate::ecs::*;

#[derive(Component)]
pub struct RigidBody {
    pub body: RigidBodyHandle,
    pub collider: ColliderHandle,
}

impl RigidBody {
    pub fn dynamic(
        rapier: ResMut<RapierWorld>,
        collider: Collider,
        position: nalgebra::Vector2<f32>,
        rotation: f32,
    ) -> Self {
        let rapier = rapier.into_inner();
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(Vector::new(position.x, position.y))
            .rotation(rotation)
            .build();
        let body_handle = rapier.bodies.insert(rigid_body);
        let collider_handle =
            rapier
                .colliders
                .insert_with_parent(collider, body_handle, &mut rapier.bodies);

        RigidBody {
            body: body_handle,
            collider: collider_handle,
        }
    }

    pub fn fixed(
        rapier: ResMut<RapierWorld>,
        collider: Collider,
        position: nalgebra::Vector2<f32>,
        rotation: f32,
    ) -> Self {
        let rapier = rapier.into_inner();
        let rigid_body = RigidBodyBuilder::fixed()
            .translation(Vector::new(position.x, position.y))
            .rotation(rotation)
            .build();
        let body_handle = rapier.bodies.insert(rigid_body);
        let collider_handle =
            rapier
                .colliders
                .insert_with_parent(collider, body_handle, &mut rapier.bodies);

        RigidBody {
            body: body_handle,
            collider: collider_handle,
        }
    }

    pub fn despawn(&self, rapier: ResMut<RapierWorld>) {
        let rapier = rapier.into_inner();
        rapier.colliders.remove(
            self.collider,
            &mut rapier.island_manager,
            &mut rapier.bodies,
            true,
        );
        rapier.bodies.remove(
            self.body,
            &mut rapier.island_manager,
            &mut rapier.colliders,
            &mut rapier.impulse_joints,
            &mut rapier.multibody_joints,
            true,
        );
    }
}

pub fn transfer_colliders(
    mut query: Query<(&mut RigidBody, &mut Transform)>,
    rapier: Res<RapierWorld>,
) {
    let rapier = rapier.into_inner();
    for (collider, mut transform) in query.iter_mut() {
        if let Some(rb) = rapier.bodies.get(collider.body) {
            let pos = rb.position().translation;
            transform.position = nalgebra::Vector3::new(pos.x, pos.y, transform.position.z);
            transform.rotation = rb.rotation().angle().to_degrees();
        }
    }
}
