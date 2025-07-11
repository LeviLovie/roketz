use bevy_ecs::prelude::*;
use rapier2d::prelude::*;

use crate::r::PhysicsWorld;

#[derive(Resource, Default)]
pub struct Collisions(pub Vec<CollisionEvent>);

pub fn init_collisions(mut commands: Commands) {
    commands.insert_resource(Collisions::default());
}

pub fn collect_collisions(physics: ResMut<PhysicsWorld>, mut collisions: ResMut<Collisions>) {
    collisions.0.clear();
    while let Ok(event) = physics.collision_events.try_recv() {
        collisions.0.push(event.clone());
    }
}
