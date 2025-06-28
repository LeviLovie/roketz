use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::r::PhysicsWorld;

#[derive(Component)]
pub struct PhysicsBody {
    pub handle: RigidBodyHandle,
}

#[derive(Component)]
pub struct PhysicsCollider {
    pub handle: ColliderHandle,
}

pub fn render_physics(query: Query<&PhysicsBody>, physics: Res<PhysicsWorld>) {
    for physics_body in query.iter() {
        if let Some(rigid_body) = physics.bodies.get(physics_body.handle) {
            let pos = rigid_body.position().translation;

            for collider_handle in rigid_body.colliders() {
                if let Some(collider) = physics.colliders.get(*collider_handle) {
                    let shape = collider.shape();

                    if let Some(ball) = shape.as_any().downcast_ref::<Ball>() {
                        draw_circle(pos.x as f32, pos.y as f32, ball.radius as f32, WHITE);
                    } else if let Some(cuboid) = shape.as_any().downcast_ref::<Cuboid>() {
                        draw_rectangle(
                            (pos.x - cuboid.half_extents.x) as f32,
                            (pos.y - cuboid.half_extents.y) as f32,
                            (cuboid.half_extents.x * 2.0) as f32,
                            (cuboid.half_extents.y * 2.0) as f32,
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
