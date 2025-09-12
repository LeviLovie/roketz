// Object data structure for rendering

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Object {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
}
