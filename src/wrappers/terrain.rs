use anyhow::{Context, Result};
use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue},
    prelude::*,
};
use std::sync::{Arc, Mutex};
use tracing::{debug, trace, warn};

use super::Camera;
use crate::{bvh::BVH, game::GameData};
use assets::Terrain as TerrainData;

pub struct Terrain {
    data: Arc<Mutex<GameData>>,
    pub width: u16,
    pub height: u16,
    pub(super) bvh: BVH,

    terrain_texture: Texture2D,
    mask_image: Image,
    mask_texture: Texture2D,
    material: Material,
    update_uniforms: bool,
    destructions: Vec<(u32, u32, u32)>,
    pub destruction_radius: u32,
}

impl Terrain {
    pub fn new(data: Arc<Mutex<GameData>>, terrain_data: &TerrainData) -> Result<Self> {
        let terrain_image = Image::from_file_with_format(
            terrain_data.texture.as_slice(),
            Some(macroquad::prelude::ImageFormat::Png),
        )?;
        let terrain_texture = Texture2D::from_image(&terrain_image);
        terrain_texture.set_filter(FilterMode::Nearest);

        let terrain_map_image = Image::from_file_with_format(
            terrain_data.map.as_slice(),
            Some(macroquad::prelude::ImageFormat::Png),
        )?;
        let terrain_map_texture = Texture2D::from_image(&terrain_map_image);
        terrain_map_texture.set_filter(FilterMode::Nearest);

        let (width, height) =
            Self::check_terrain_texture_sizes(&terrain_texture, &terrain_map_texture);
        debug!("Creating terrain with size: {}x{}", width, height);

        let bvh_depth = data.lock().unwrap().config.physics.bvh_depth as usize;
        let mut bvh = BVH::new(width as u32, height as u32, bvh_depth);

        trace!("Destructing terrain");
        for y in 0..height {
            for x in 0..width {
                let pixel = terrain_map_image.get_pixel(x as u32, y as u32);
                if pixel.r <= 0.1 && pixel.g <= 0.1 && pixel.b <= 0.1 {
                    bvh.cut_point(vec2(x as f32, y as f32));
                }
            }
        }
        debug!("Terrain destructed");

        let shader = data
            .lock()
            .unwrap()
            .assets
            .get_asset::<assets::Shader>("TerrainShader")
            .context("Failed to get terrain shader")?
            .clone();
        let material = load_material(
            ShaderSource::Glsl {
                vertex: String::from_utf8(shader.vertex.clone())
                    .context("Invalid vertex shader")?
                    .as_str(),
                fragment: String::from_utf8(shader.fragment.clone())
                    .context("Invalid fragment shader")?
                    .as_str(),
            },
            MaterialParams {
                pipeline_params: PipelineParams {
                    depth_write: false,
                    color_blend: Some(BlendState::new(
                        miniquad::Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    alpha_blend: Some(BlendState::new(
                        miniquad::Equation::Add,
                        BlendFactor::One,
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
                textures: vec!["tex".to_string(), "mask".to_string()],
            },
        )?;

        debug!("Terrain crated");
        Ok(Self {
            data: data.clone(),
            width,
            height,
            bvh,

            terrain_texture,
            mask_image: terrain_map_image,
            mask_texture: terrain_map_texture,
            material,
            update_uniforms: true,
            destructions: Vec::new(),
            destruction_radius: 10,
        })
    }

    fn check_terrain_texture_sizes(
        terrain_texture: &Texture2D,
        terrain_map_texture: &Texture2D,
    ) -> (u16, u16) {
        let terrain_texture_width = terrain_texture.width() as u16;
        let terrain_texture_height = terrain_texture.height() as u16;
        let terrain_map_texture_width = terrain_map_texture.width() as u16;
        let terrain_map_texture_height = terrain_map_texture.height() as u16;

        if terrain_texture_width != terrain_map_texture_width {
            warn!(
                "Terrain texture width ({}) does not match terrain map texture width ({}). Continuing with the smaller size.",
                terrain_texture_width, terrain_map_texture_width
            );
        }
        if terrain_texture_height != terrain_map_texture_height {
            warn!(
                "Terrain texture height ({}) does not match terrain map texture height ({}). Continuing with the smaller size.",
                terrain_texture_height, terrain_map_texture_height
            );
        }

        let width = terrain_texture_width.min(terrain_map_texture_width);
        let height = terrain_texture_height.min(terrain_map_texture_height);

        (width, height)
    }

    pub fn destruct(&mut self, loc_x: u32, loc_y: u32, radius: u32) {
        self.bvh
            .cut_circle(vec2(loc_x as f32, loc_y as f32), radius as f32);

        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as i32 - loc_x as i32;
                let dy = y as i32 - loc_y as i32;
                if dx * dx + dy * dy <= radius as i32 * radius as i32 {
                    self.mask_image.set_pixel(x.into(), y.into(), BLACK);
                }
            }
        }
        self.update_uniforms = true;

        self.destructions.push((loc_x, loc_y, radius));
    }

    pub fn update(&mut self) {
        if self.update_uniforms {
            self.update_uniforms = false;
            self.mask_texture.update(&self.mask_image);
            self.mask_texture.set_filter(FilterMode::Nearest);
            self.material
                .set_texture("tex", self.terrain_texture.clone());
            self.material.set_texture("mask", self.mask_texture.clone());
        }
    }

    pub fn draw(&self, camera: &Camera) {
        gl_use_material(&self.material);
        let zoom = Camera::zoom_vec(camera.zoom);
        let top_left = vec2(0.0, 0.0);
        let screen_pos = (top_left - camera.target) * zoom;
        draw_texture_ex(
            &self.terrain_texture,
            screen_pos.x,
            screen_pos.y * -1.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    zoom.x * self.width as f32,
                    zoom.y * self.height as f32 * -1.0,
                )),
                ..Default::default()
            },
        );
        gl_use_default_material();

        if self.data.lock().unwrap().debug.ol_bvh {
            self.bvh.draw();

            for (loc_x, loc_y, radius) in &self.destructions {
                draw_circle_lines(
                    *loc_x as f32,
                    *loc_y as f32,
                    *radius as f32,
                    0.5,
                    Color::new(0.0, 0.0, 1.0, 0.25),
                );
            }
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if self.data.lock().unwrap().debug.v_terrain {
            egui::Window::new("Terrain")
                .default_pos((10.0, 10.0))
                .show(ctx, |ui| {
                    ui.label(format!("Size: {}x{}", self.width, self.height));
                });
        }
    }
}
