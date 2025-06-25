use bevy_ecs::prelude::*;

use crate::components::Transform;

pub fn update_transforms(mut query: Query<&mut Transform>) {
    for mut t in query.iter_mut() {
        let acc = t.acc;
        let vel = t.vel + acc;
        t.vel = vel;
        t.pos += vel;
    }
}
