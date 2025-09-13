mod camera;
mod device;
mod gbuffer;
mod object;
mod passes;
mod texture_cache;

use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

use gbuffer::GBuffer;

pub use camera::Camera;
pub use object::Object;
pub use texture_cache::{GpuTextureHandle, TextureCache, load_texture_from_disk};

const LAYERS: u32 = 4;
const COMPOSITE_MODE: passes::CompositeMode = passes::CompositeMode::Composite;

pub struct Renderer {
    pub window: Arc<Window>,
    pub device: Device,
    pub queue: Queue,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    gbuffer: GBuffer,
    geometry_pass: passes::Geometry,
    composite_pass: passes::Composite,
    pub texture_cache: TextureCache,
    pub textures_updated: bool,
    layer_zs: Vec<f32>,
    layers: Vec<Vec<Object>>,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let (device, queue, surface, config) = device::init_wgpu(window.clone()).await;

        let texture_cache = TextureCache::new(&device);

        let gbuffer = GBuffer::new(&device, config.width, config.height, LAYERS);
        let geometry_pass = passes::Geometry::new(&device, &gbuffer, &texture_cache);
        let composite_pass =
            passes::Composite::new(&device, config.format, &gbuffer, COMPOSITE_MODE);

        Self {
            window,
            device,
            queue,
            surface,
            config,
            gbuffer,
            geometry_pass,
            composite_pass,
            texture_cache,
            textures_updated: false,
            layer_zs: vec![-25.0, -10.0, 0.0, 10.0],
            layers: vec![vec![], vec![], vec![], vec![]],
        }
    }

    pub fn clear_layers(&mut self) {
        self.layers.iter_mut().for_each(|layer| layer.clear());
    }

    pub fn push_object(&mut self, layer: u32, object: Object) {
        match self.layers.get_mut(layer as usize) {
            Some(objects) => {
                objects.push(object);
            }
            None => {
                println!("Layer {} does not exist", layer);
            }
        }
    }

    pub fn render(&mut self, camera: &Camera) {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Lost) => {
                self.resize(self.config.width, self.config.height);
                return;
            }
            Err(wgpu::SurfaceError::OutOfMemory) => std::process::exit(1),
            Err(_) => return,
        };
        let surface_view = frame.texture.create_view(&Default::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());

        if self.textures_updated {
            self.texture_cache.build_batches(&self.device, &self.queue);
        }

        let mut rpd = passes::RenderPassData {
            texture_cache: &mut self.texture_cache,
            gbuffer: &self.gbuffer,
            encoder: &mut encoder,
            device: &self.device,
            queue: &self.queue,
        };

        for (i, objects) in self.layers.iter().enumerate().take(LAYERS as usize) {
            if objects.is_empty() {
                continue;
            }
            let z = self.layer_zs.get(i).cloned().unwrap_or(0.0);
            self.geometry_pass
                .execute(&mut rpd, objects, i as u32, z, camera);
        }

        self.composite_pass.execute(&mut rpd, &surface_view);

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
        self.geometry_pass =
            passes::Geometry::new(&self.device, &self.gbuffer, &self.texture_cache);
        self.composite_pass = passes::Composite::new(
            &self.device,
            self.config.format,
            &self.gbuffer,
            COMPOSITE_MODE,
        );
    }
}
