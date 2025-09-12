#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub z: f32,
    _pad: u32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: [0.0, 0.0],
            size: [2.0, 2.0],
            z: -10.0,
            _pad: 0,
        }
    }
}

impl Camera {
    pub fn new(pos: [f32; 2], size: [f32; 2], z: f32) -> Self {
        Self {
            pos,
            size,
            z,
            _pad: 0,
        }
    }
}
