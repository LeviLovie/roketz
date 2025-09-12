use nalgebra::Vector2;

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
            size: [10.0, 10.0],
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

    pub fn set_position(&mut self, pos: Vector2<f32>) {
        self.pos = pos.into();
    }

    pub fn set_center(&mut self, center: Vector2<f32>) {
        self.pos = (center - Vector2::from(self.size) / 2.0).into();
    }
}
