use bevy_ecs::prelude::Component;
use nalgebra::{Vector2, Vector3};

#[derive(Component)]
pub struct Transform {
    pub layer: u32,
    pub position: Vector3<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
}
