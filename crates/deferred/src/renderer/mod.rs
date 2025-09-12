mod device;
mod gbuffer;
pub mod camera;
pub mod object;
mod passes;

use camera::Camera;
use object::Object;
use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

use gbuffer::GBuffer;

const LAYERS: u32 = 4;
const COMPOSITE_MODE: passes::CompositeMode = passes::CompositeMode::Composite;
// const COMPOSITE_MODE: passes::CompositeMode = passes::CompositeMode::Grid;

pub struct Renderer {
    pub window: Arc<Window>,
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    gbuffer: GBuffer,
    geometry_pass: passes::Geometry,
    composite_pass: passes::Composite,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let (device, queue, surface, config) = device::init_wgpu(window.clone()).await;

        let gbuffer = GBuffer::new(&device, config.width, config.height, LAYERS);
        let geometry_pass = passes::Geometry::new(&device, &gbuffer);
        let composite_pass = passes::Composite::new(&device, config.format, &gbuffer, COMPOSITE_MODE);

        Self {
            window,
            device,
            queue,
            surface,
            config,
            gbuffer,
            geometry_pass,
            composite_pass,
        }
    }

    pub fn render(&mut self, objects: Vec<(f32, Vec<Object>)>, camera: &Camera) {
        let frame = self.surface.get_current_texture().unwrap();
        let surface_view = frame.texture.create_view(&Default::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());

        let mut rpd = passes::RenderPassData {
            gbuffer: &self.gbuffer,
            encoder: &mut encoder,
            device: &self.device,
            queue: &self.queue,
        };

        for (i, (z, objects)) in objects.iter().enumerate()
        {
            if i as u32 >= LAYERS {
                break;
            }
            if objects.is_empty() {
                continue;
            }

            self.geometry_pass
                .execute(&mut rpd, objects, i as u32, *z, &camera);
        }
        {
            self.composite_pass.execute(&mut rpd, &surface_view);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);

        self.gbuffer = GBuffer::new(&self.device, width, height, LAYERS);
        self.geometry_pass = passes::Geometry::new(&self.device, &self.gbuffer);
        self.composite_pass =
            passes::Composite::new(&self.device, self.config.format, &self.gbuffer, COMPOSITE_MODE);
    }
}
