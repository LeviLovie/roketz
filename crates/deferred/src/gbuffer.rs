use wgpu::{
    Device, Extent3d, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    TextureView, TextureViewDescriptor, TextureViewDimension,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Frame {
    pub size: [u32; 2],
}

pub struct GBuffer {
    pub color_texture: Texture,
    pub color_view: TextureView,
    pub depth_texture: Texture,
    pub format: TextureFormat,
    pub depth_format: TextureFormat,
    pub layers: u32,
    pub size: (u32, u32),
}

impl GBuffer {
    pub fn new(device: &Device, width: u32, height: u32, layers: u32) -> Self {
        let format = TextureFormat::Rgba8Unorm;
        let depth_format = TextureFormat::Depth24Plus;

        let color_texture = device.create_texture(&TextureDescriptor {
            label: Some("GBuffer Color Array"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: layers,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let color_view = color_texture.create_view(&TextureViewDescriptor {
            label: Some("GBuffer Color View"),
            dimension: Some(TextureViewDimension::D2Array),
            base_array_layer: 0,
            array_layer_count: Some(layers),
            ..Default::default()
        });

        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("GBuffer Depth Array"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: layers,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: depth_format,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        Self {
            color_texture,
            color_view,
            depth_texture,
            format,
            depth_format,
            size: (width, height),
            layers,
        }
    }

    pub fn frame(&self) -> Frame {
        Frame {
            size: [self.size.0, self.size.1],
        }
    }

    pub fn color_layer_view(&self, layer: u32) -> TextureView {
        self.color_texture.create_view(&TextureViewDescriptor {
            label: Some(&format!("GBuffer Color Layer {}", layer)),
            dimension: Some(TextureViewDimension::D2),
            base_array_layer: layer,
            array_layer_count: Some(1),
            ..Default::default()
        })
    }

    pub fn depth_layer_view(&self, layer: u32) -> TextureView {
        self.depth_texture.create_view(&TextureViewDescriptor {
            label: Some(&format!("GBuffer Depth Layer {}", layer)),
            dimension: Some(TextureViewDimension::D2),
            base_array_layer: layer,
            array_layer_count: Some(1),
            ..Default::default()
        })
    }
}
