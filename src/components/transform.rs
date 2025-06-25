use macroquad::prelude::*;

#[derive(Debug)]
pub struct Transform {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub angle: f32,
}
