use bevy_ecs::prelude::*;
use macroquad::prelude::*;

#[derive(Component, Debug)]
pub struct Transform {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub angle: f32,
}

impl Transform {
    pub fn from_pos(pos: Vec2) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            angle: 0.0,
        }
    }
}
