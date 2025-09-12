use std::sync::Arc;

use wgpu::{
    CompositeAlphaMode, Device, Instance, PresentMode, Queue, RequestAdapterOptions, Surface,
    SurfaceConfiguration, TextureUsages,
};
use winit::window::Window;

pub async fn init_wgpu(
    window: Arc<Window>,
) -> (Device, Queue, Surface<'static>, SurfaceConfiguration) {
    let size = window.inner_size();

    let instance = Instance::default();
    let surface = instance.create_surface(window).unwrap();

    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .unwrap();

    let (device, queue) = adapter.request_device(&Default::default()).await.unwrap();

    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_capabilities(&adapter).formats[0],
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    (device, queue, surface, config)
}
