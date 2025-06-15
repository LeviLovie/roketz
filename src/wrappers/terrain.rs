use anyhow::{Context, Result};
use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue},
    prelude::*,
};
use std::sync::{Arc, Mutex};
use tracing::{debug, error};

use super::Camera;
use crate::{bvh::BVH, game::GameData};

pub struct Terrain {
    data: Arc<Mutex<GameData>>,
    pub width: u16,
    pub height: u16,
    bvh: BVH,

    terrain_texture: Texture2D,
    mask_image: Image,
    mask_texture: Texture2D,
    material: Material,
    update_uniforms: bool,
    destructions: Vec<(u32, u32, u32)>,
}

impl Terrain {
    pub fn new(data: Arc<Mutex<GameData>>) -> Result<Self> {
        let texture = data
            .lock()
            .unwrap()
            .assets
            .get_asset::<assets::Texture>("TerrainTexture")
            .context("Failed to get terrain texture")?
            .clone();
        let width = texture.width as u16;
        let height = texture.height as u16;
        debug!("Creating terrain with size: {}x{}", width, height);
        let image = Image::from_file_with_format(
            texture.texture.as_slice(),
            Some(macroquad::prelude::ImageFormat::Png),
        )?;
        let terrain_texture = Texture2D::from_image(&image);
        terrain_texture.set_filter(FilterMode::Nearest);

        let mask_image = Image::gen_image_color(width, height, WHITE);
        let mask_texture = Texture2D::from_image(&mask_image);
        mask_texture.set_filter(FilterMode::Nearest);

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

        let bvh_depth = data.lock().unwrap().config.physics.bvh_depth as usize;
        let bvh = BVH::new(width as u32, height as u32, bvh_depth);

        debug!("Terrain crated");
        let mut terrain = Self {
            data: data.clone(),
            width,
            height,
            bvh,

            terrain_texture,
            mask_image,
            mask_texture,
            material,
            update_uniforms: true,
            destructions: Vec::new(),
        };

        let destructions = String::from_utf8(
            data.clone()
                .lock()
                .unwrap()
                .assets
                .get_asset::<assets::Destructions>("TerrainDestructions")
                .context("Failed to get destructions")?
                .data
                .clone(),
        )?;
        for destruction_line in destructions.lines() {
            let parts = destruction_line
                .split(',')
                .map(|s| s.trim().parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to parse destruction line")?;
            let loc_x = parts.get(0).cloned().unwrap_or(0);
            let loc_y = parts.get(1).cloned().unwrap_or(0);
            let radius = parts.get(2).cloned().unwrap_or(0);
            terrain.destruct(loc_x, loc_y, radius);
        }

        Ok(terrain)
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
        #[cfg(debug_assertions)]
        {
            if is_key_pressed(KeyCode::P) {
                let mut dump = String::new();
                for (loc_x, loc_y, radius) in &self.destructions {
                    dump.push_str(&format!("{}, {}, {}\n", loc_x, loc_y, radius));
                }
                if let Err(e) = std::fs::write("assets/destructions.csv", dump) {
                    error!("Failed to write destructions: {}", e);
                } else {
                    debug!("Destructions written to assets/destructions.csv");
                }
            }

            if is_key_pressed(KeyCode::O) {
                self.bvh = BVH::new(
                    self.width as u32,
                    self.height as u32,
                    self.data.lock().unwrap().config.physics.bvh_depth as usize,
                );
                self.destructions.clear();
                self.mask_image = Image::gen_image_color(self.width, self.height, WHITE);
                self.update_uniforms = true;
            }
        }

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

        if self.data.lock().unwrap().is_debug {
            self.bvh.draw();

            for (loc_x, loc_y, radius) in &self.destructions {
                draw_circle(
                    *loc_x as f32,
                    *loc_y as f32,
                    *radius as f32,
                    Color::new(0.0, 0.0, 1.0, 0.25),
                );
            }
        }
    }
}
