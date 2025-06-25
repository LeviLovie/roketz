use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use super::{Physics, Transform};
use crate::ecs::res::{Gravity, DT};

#[derive(Component, Debug)]
pub struct Player {
    pub color: Color,
    pub thrust: f32,
    pub rotation_speed: f32,
}

impl Player {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            thrust: 150.0,
            rotation_speed: 5.0,
        }
    }
}

pub fn update_players(
    mut query: Query<(&Player, &mut Transform, &mut Physics)>,
    dt: Res<DT>,
    gravity: Res<Gravity>,
) {
    for (player, mut transform, mut physics) in query.iter_mut() {
        if is_key_down(KeyCode::A) {
            transform.angle -= player.rotation_speed * dt.0;
        } else if is_key_down(KeyCode::D) {
            transform.angle += player.rotation_speed * dt.0;
        }

        let mut force = Vec2::ZERO;
        if is_key_down(KeyCode::W) {
            force += vec2(transform.angle.cos(), transform.angle.sin()) * player.thrust;
        }
        force.y += gravity.0 * physics.mass;

        physics.acc = force / physics.mass;
        let dv = physics.acc * dt.0;
        let drag = physics.drag;
        physics.vel += dv;
        physics.vel *= drag;
        transform.pos += physics.vel * dt.0;

        if physics.vel.length() < 1.0 {
            physics.vel = Vec2::ZERO;
        }
    }
}

pub fn draw_players(query: Query<(&Player, &Transform)>) {
    for (p, t) in query.iter() {
        draw_circle(t.pos.x, t.pos.y, 10.0, p.color);
        draw_line(
            t.pos.x,
            t.pos.y,
            t.pos.x + t.angle.cos() * 20.0,
            t.pos.y + t.angle.sin() * 20.0,
            2.0,
            p.color,
        );
    }
}
