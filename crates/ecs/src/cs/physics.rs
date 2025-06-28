use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::{cs::Transform, r::DT};

#[derive(Component)]
pub struct Physics {
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub drag: f32,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            mass: 1.0,
            drag: 0.9975,
        }
    }
}

pub fn update_physics(mut query: Query<(&mut Physics, &mut Transform)>, dt: Res<DT>) {
    for (mut p, mut t) in query.iter_mut() {
        let acc = p.acc / p.mass;
        let vel = (p.vel + acc * dt.0) * p.drag;
        p.vel = vel;
        t.pos += vel * dt.0;

        if p.vel.length() < 1.0 {
            p.vel = Vec2::ZERO;
        }
    }
}
