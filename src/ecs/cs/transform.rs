use bevy_ecs::prelude::*;
use macroquad::prelude::*;

#[derive(Component, Debug)]
pub struct Transform {
    pub pos: Vec2,
    pub angle: f32,
}

impl Transform {
    pub fn from_pos(pos: Vec2) -> Self {
        Self { pos, angle: 0.0 }
    }
}
