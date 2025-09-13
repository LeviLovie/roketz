use std::{collections::HashMap, path::Path};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Device, Extent3d, FilterMode, Origin3d,
    Queue, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, TexelCopyBufferLayout,
    TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor, TextureDimension,
    TextureFormat, TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor,
    TextureViewDimension,
};

const MAX_TEXTURES_PER_BATCH: usize = 16;

#[derive(Clone)]
pub struct GpuTextureHandle {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
    pub rgba_data: Vec<u8>,
}

pub struct TextureBatch {
    pub bind_group: BindGroup,
    pub textures: Vec<GpuTextureHandle>,
}

pub struct TextureCache {
    pub textures: Vec<(GpuTextureHandle, u32, u32)>,
    pub lookup: HashMap<String, u32>,
    pub id_to_batch: HashMap<u32, (usize, u32)>,
    pub batches: Vec<TextureBatch>,
    pub bind_group_layout: BindGroupLayout,
}

#[allow(dead_code)]
impl TextureCache {
    pub fn new(device: &Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Texture batch BGL"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        Self {
            textures: Vec::new(),
            lookup: HashMap::new(),
            id_to_batch: HashMap::new(),
            batches: Vec::new(),
            bind_group_layout,
        }
    }

    pub fn load_texture(&mut self, device: &Device, queue: &Queue, path: &str) -> u32 {
        if let Some(&id) = self.lookup.get(path) {
            return id;
        }

        let (handle, width, height) = load_texture_from_disk(device, queue, path);
        let id = self.textures.len() as u32;
        self.textures.push((handle, width, height));
        self.lookup.insert(path.to_string(), id);

        id
    }

    fn create_texture_array(
        device: &Device,
        queue: &Queue,
        textures: &[(GpuTextureHandle, u32, u32)],
    ) -> GpuTextureHandle {
        let width = textures[0].1;
        let height = textures[0].2;
        let layer_count = textures.len() as u32;

        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: layer_count,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Texture Array"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        for (i, tex) in textures.iter().enumerate() {
            queue.write_texture(
                TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: Origin3d {
                        x: 0,
                        y: 0,
                        z: i as u32,
                    },
                    aspect: TextureAspect::All,
                },
                &tex.0.rgba_data,
                TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * width),
                    rows_per_image: Some(height),
                },
                Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
        }

        let view = texture.create_view(&TextureViewDescriptor {
            dimension: Some(TextureViewDimension::D2Array),
            ..Default::default()
        });

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("Texture Array Sampler"),
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        GpuTextureHandle {
            texture,
            view,
            sampler,
            rgba_data: Vec::new(),
        }
    }

    pub fn build_batches(&mut self, device: &Device, queue: &Queue) {
        self.batches.clear();
        self.id_to_batch.clear();

        type TexSize = (u32, u32);
        type TexInfo = (GpuTextureHandle, u32, u32);

        let mut grouped: HashMap<(u32, u32), Vec<(u32, TexInfo)>> = HashMap::new();

        for (id, tex) in self.textures.iter().enumerate() {
            grouped
                .entry((tex.1, tex.2))
                .or_default()
                .push((id as u32, tex.clone()));
        }

        let mut groups: Vec<(TexSize, Vec<(u32, TexInfo)>)> = grouped.into_iter().collect();
        groups.sort_by(|a, b| a.0.cmp(&b.0));

        for (_size, mut group) in groups {
            group.sort_by_key(|(tex_id, _)| *tex_id);

            for chunk in group.chunks(MAX_TEXTURES_PER_BATCH) {
                let tex_infos: Vec<TexInfo> = chunk.iter().map(|(_, t)| t.clone()).collect();
                let array_handle = Self::create_texture_array(device, queue, &tex_infos);

                let bind_group = device.create_bind_group(&BindGroupDescriptor {
                    label: Some("Texture Array BG"),
                    layout: &self.bind_group_layout,
                    entries: &[
                        BindGroupEntry {
                            binding: 0,
                            resource: BindingResource::TextureView(&array_handle.view),
                        },
                        BindGroupEntry {
                            binding: 1,
                            resource: BindingResource::Sampler(&array_handle.sampler),
                        },
                    ],
                });

                let batch_index = self.batches.len();

                for (layer_index, (tex_id, _)) in chunk.iter().enumerate() {
                    self.id_to_batch
                        .insert(*tex_id, (batch_index, layer_index as u32));
                }

                self.batches.push(TextureBatch {
                    bind_group,
                    textures: vec![array_handle],
                });
            }
        }
    }

    pub fn get_batch_info(&self, texture_id: u32) -> Option<(usize, u32)> {
        self.id_to_batch.get(&texture_id).copied()
    }

    pub fn get_texture_id(&self, path: &str) -> Option<u32> {
        self.lookup.get(path).copied()
    }

    pub fn get_bind_group(&self, batch_index: usize) -> Option<&BindGroup> {
        self.batches.get(batch_index).map(|b| &b.bind_group)
    }
}

pub fn load_texture_from_disk(
    device: &Device,
    queue: &Queue,
    path: &str,
) -> (GpuTextureHandle, u32, u32) {
    let img = image::open(Path::new(path)).expect("Failed to load texture");
    let rgba = img.to_rgba8();
    let (width, height) = (img.width(), img.height());

    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&TextureDescriptor {
        label: Some(path),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        &rgba,
        TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        },
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    let view = texture.create_view(&TextureViewDescriptor::default());
    let sampler = device.create_sampler(&SamplerDescriptor {
        label: Some(&format!("{} sampler", path)),
        mag_filter: FilterMode::Nearest,
        min_filter: FilterMode::Nearest,
        mipmap_filter: FilterMode::Nearest,
        ..Default::default()
    });

    (
        GpuTextureHandle {
            texture,
            view,
            sampler,
            rgba_data: rgba.to_vec(),
        },
        width,
        height,
    )
}
