mod composite;
mod geometry;

pub use composite::{Composite, CompositeMode};
pub use geometry::Geometry;

use wgpu::{CommandEncoder, Device, Queue};

use super::gbuffer::GBuffer;

pub struct RenderPassData<'a> {
    pub gbuffer: &'a GBuffer,
    pub encoder: &'a mut CommandEncoder,
    pub device: &'a Device,
    pub queue: &'a Queue,
}
