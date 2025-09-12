use getset::{Getters, MutGetters, Setters};
use nalgebra::{Vector2, Vector4};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ObjectRaw {
    pub pos: [f32; 2],  // 8 bytes
    pub size: [f32; 2], // 8 bytes
    pub tint: [u8; 4],  // 4 bytes
    pub rot: f32,       // 4 bytes
    pub bid: u32,       // 4 bytes
    pub tid: u32,       // 4 bytes
                        // 32 bytes total, no padding needed
}

#[derive(Getters, Setters, MutGetters)]
#[get = "pub"]
#[set = "pub"]
#[get_mut = "pub"]
pub struct Object {
    pub pos: Vector2<f32>,
    pub size: Vector2<f32>,
    pub rot: f32,
    pub tint: Vector4<u8>,
    bid: usize,
    tid: usize,
}

impl From<&Object> for ObjectRaw {
    fn from(val: &Object) -> Self {
        ObjectRaw {
            pos: val.pos.into(),
            size: val.size.into(),
            rot: val.rot,
            tint: val.tint.into(),
            bid: val.bid as u32,
            tid: val.tid as u32,
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Self {
            pos: Vector2::new(0.0, 0.0),
            size: Vector2::new(1.0, 1.0),
            rot: 0.0,
            tint: Vector4::new(255, 255, 255, 255),
            bid: 0,
            tid: 0,
        }
    }
}
