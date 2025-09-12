use std::collections::HashMap;

use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BlendState, Buffer, BufferBindingType, BufferUsages, Color,
    ColorTargetState, ColorWrites, CompareFunction, DepthStencilState, Device, FragmentState,
    LoadOp, Operations, PipelineLayoutDescriptor, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderStages, StoreOp, VertexState,
};

use crate::{object::ObjectRaw, TextureCache};

use super::super::{camera::Camera, gbuffer::GBuffer, object::Object};
use super::RenderPassData;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Layer {
    z: f32,
    _pad: [u32; 3],
}

pub struct Geometry {
    pipeline: RenderPipeline,
    objects_bgl: BindGroupLayout,
    params_bgl: BindGroupLayout,
    frame_b: Buffer,
    camera_b: Buffer,
}

impl Geometry {
    pub fn new(device: &Device, gbuffer: &GBuffer, textures_cache: &TextureCache) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../shaders/geometry.wgsl"));

        let objects_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Object Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let frame = gbuffer.frame();
        let frame_b = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Frame B"),
            contents: bytemuck::cast_slice(&[frame]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let camera = Camera::default();
        let camera_b = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera B"),
            contents: bytemuck::cast_slice(&[camera]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let params_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Params BGL"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Geometry Pipeline Layout"),
            bind_group_layouts: &[&objects_bgl, &params_bgl, &textures_cache.bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Geometry Pipeline"),
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
                    format: gbuffer.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: Some(DepthStencilState {
                format: gbuffer.depth_format,
                depth_write_enabled: false,
                depth_compare: CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            objects_bgl,
            params_bgl,
            frame_b,
            camera_b,
        }
    }

    pub fn execute(
        &self,
        data: &mut RenderPassData,
        objects: &[Object],
        layer: u32,
        layer_z: f32,
        camera: &Camera,
    ) {
        data.queue
            .write_buffer(&self.camera_b, 0, bytemuck::cast_slice(&[*camera]));
        data.queue.write_buffer(
            &self.frame_b,
            0,
            bytemuck::cast_slice(&[data.gbuffer.frame()]),
        );

        let layer_b = data.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Layer B"),
            contents: bytemuck::cast_slice(&[Layer {
                z: layer_z,
                _pad: [0; 3],
            }]),
            usage: BufferUsages::UNIFORM,
        });

        let params_bg = data.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Params BG"),
            layout: &self.params_bgl,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.frame_b.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: self.camera_b.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: layer_b.as_entire_binding(),
                },
            ],
        });

        let mut batch_groups: HashMap<usize, Vec<&Object>> = HashMap::new();
        for obj in objects {
            batch_groups.entry(*obj.bid()).or_default().push(obj);
        }

        for (batch_id, batch_objects) in batch_groups {
            let objects_data: Vec<ObjectRaw> =
                batch_objects.iter().map(|obj| (*obj).into()).collect();
            let objects_b = data.device.create_buffer_init(&BufferInitDescriptor {
                label: Some(&format!("Object Buffer Batch {}", batch_id)),
                contents: bytemuck::cast_slice(&objects_data),
                usage: BufferUsages::STORAGE,
            });
            let objects_bg = data.device.create_bind_group(&BindGroupDescriptor {
                label: Some("Object Bind Group"),
                layout: &self.objects_bgl,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: objects_b.as_entire_binding(),
                }],
            });

            let color_view = &data.gbuffer.color_layer_view(layer);
            let depth_view = &data.gbuffer.depth_layer_view(layer);

            let mut rpass = data.encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some(&format!("Geometry Pass Layer {} Batch {}", layer, batch_id)),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: color_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::TRANSPARENT),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: depth_view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });

            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &objects_bg, &[]);
            rpass.set_bind_group(1, &params_bg, &[]);
            rpass.set_bind_group(2, &data.texture_cache.batches[batch_id].bind_group, &[]);
            rpass.draw(0..6, 0..batch_objects.len() as u32);
        }
    }
}
