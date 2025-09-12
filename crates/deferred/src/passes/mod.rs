mod composite;
mod geometry;

pub use composite::{Composite, CompositeMode};
pub use geometry::Geometry;

use wgpu::{CommandEncoder, Device, Queue};

use super::{gbuffer::GBuffer, TextureCache};

pub struct RenderPassData<'a> {
    pub texture_cache: &'a mut TextureCache,
    pub gbuffer: &'a GBuffer,
    pub encoder: &'a mut CommandEncoder,
    pub device: &'a Device,
    pub queue: &'a Queue,
}
