use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{
    cs::Transform,
    r::{Debug, PhysicsWorld},
};

#[derive(Component)]
pub struct RigidCollider {
    pub body: RigidBodyHandle,
    pub collider: ColliderHandle,
}

impl RigidCollider {
    pub fn fixed(
        physics: &mut Mut<PhysicsWorld>,
        col: Collider,
        pos: Vector<f32>,
        rot: f32,
    ) -> Self {
        let rb = RigidBodyBuilder::fixed()
            .translation(pos)
            .rotation(rot)
            .build();
        let PhysicsWorld {
            bodies, colliders, ..
        } = &mut **physics;
        let rb_handle = bodies.insert(rb);
        let col_handle = colliders.insert_with_parent(col, rb_handle, bodies);
        Self {
            body: rb_handle,
            collider: col_handle,
        }
    }

    pub fn dynamic(
        physics: &mut Mut<PhysicsWorld>,
        col: Collider,
        pos: Vector<f32>,
        rot: f32,
    ) -> Self {
        let rb = RigidBodyBuilder::dynamic()
            .translation(pos)
            .rotation(rot)
            .build();
        let PhysicsWorld {
            bodies, colliders, ..
        } = &mut **physics;
        let rb_handle = bodies.insert(rb);
        let col_handle = colliders.insert_with_parent(col, rb_handle, bodies);
        Self {
            body: rb_handle,
            collider: col_handle,
        }
    }

    pub fn despawn(&mut self, mut physics: ResMut<PhysicsWorld>) {
        let PhysicsWorld {
            bodies,
            colliders,
            island_manager,
            impulse_joints,
            multibody_joints,
            ..
        } = &mut *physics;
        colliders.remove(self.collider, island_manager, bodies, true);
        bodies.remove(
            self.body,
            island_manager,
            colliders,
            impulse_joints,
            multibody_joints,
            true,
        );
    }
}

pub fn render_colliders(
    query: Query<&RigidCollider>,
    physics: Res<PhysicsWorld>,
    debug: Res<Debug>,
) {
    if !debug.o_physics {
        return;
    }

    for collider in query.iter() {
        if let Some(rigid_body) = physics.bodies.get(collider.body) {
            let pos = rigid_body.position().translation;

            for collider_handle in rigid_body.colliders() {
                if let Some(collider) = physics.colliders.get(*collider_handle) {
                    let shape = collider.shape();

                    if let Some(ball) = shape.as_any().downcast_ref::<Ball>() {
                        draw_circle_lines(pos.x, pos.y, ball.radius, 1.0, WHITE);
                    } else if let Some(cuboid) = shape.as_any().downcast_ref::<Cuboid>() {
                        draw_rectangle_lines(
                            pos.x - cuboid.half_extents.x,
                            pos.y - cuboid.half_extents.y,
                            cuboid.half_extents.x * 2.0,
                            cuboid.half_extents.y * 2.0,
                            1.0,
                            WHITE,
                        );
                    } else {
                        println!("Unsupported collider shape for rendering.");
                    }
                }
            }
        }
    }
}

pub fn transfer_colliders(
    mut query: Query<(&mut RigidCollider, &mut Transform)>,
    physics: Res<PhysicsWorld>,
) {
    let PhysicsWorld { bodies, .. } = &*physics;
    for (collider, mut transform) in query.iter_mut() {
        if let Some(rb) = bodies.get(collider.body) {
            let pos = rb.position().translation;
            transform.pos = vec2(pos.x, pos.y);
            transform.angle = rb.position().rotation.angle();
        }
    }
}
