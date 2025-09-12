use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendState, Color, ColorTargetState,
    ColorWrites, Device, FragmentState, LoadOp, MultisampleState, Operations,
    PipelineLayoutDescriptor, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, SamplerBindingType, SamplerDescriptor, ShaderStages,
    StoreOp, TextureFormat, TextureSampleType, TextureView, TextureViewDimension, VertexState,
};

use crate::renderer::gbuffer::GBuffer;

use super::RenderPassData;

#[allow(dead_code)]
pub enum CompositeMode {
    Composite = 0,
    Grid = 1,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ParamsUniform {
    background: [f32; 3],
    mode: u32,
}

impl ParamsUniform {
    fn new(mode: CompositeMode) -> Self {
        Self {
            background: [0.1, 0.1, 0.1],
            mode: mode as u32,
        }
    }
}

pub struct Composite {
    pipeline: RenderPipeline,
    gbuffer_bg: BindGroup,
    params_bg: BindGroup,
    _layers: u32,
}

impl Composite {
    pub fn new(
        device: &Device,
        format: TextureFormat,
        gbuffer: &GBuffer,
        mode: CompositeMode,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../../shaders/composite.wgsl"));
        let sampler = device.create_sampler(&SamplerDescriptor::default());

        let gbuffer_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Composite GBuffer BGL"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: TextureViewDimension::D2Array,
                        sample_type: TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
            ],
        });
        let gbuffer_bg = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Composite Grid BG"),
            layout: &gbuffer_bgl,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&gbuffer.color_view),
                },
            ],
        });

        let params = ParamsUniform::new(mode);
        let params_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Composite Params BGL"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let params_b = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Composite Params B"),
            contents: bytemuck::bytes_of(&params),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let params_bg = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Composite Params BG"),
            layout: &params_bgl,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: params_b.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Composite PL"),
            bind_group_layouts: &[&gbuffer_bgl, &params_bgl],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Composite P"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            gbuffer_bg,
            params_bg,
            _layers: gbuffer.layers,
        }
    }

    pub fn execute(&self, data: &mut RenderPassData, view: &TextureView) {
        let mut rpass = data.encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Composite Grid Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: StoreOp::Store,
                },
                depth_slice: None,
            })],
            ..Default::default()
        });

        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.gbuffer_bg, &[]);
        rpass.set_bind_group(1, &self.params_bg, &[]);
        rpass.draw(0..6, 0..1);
    }
}
