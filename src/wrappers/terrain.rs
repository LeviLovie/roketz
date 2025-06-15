use anyhow::{Context, Result};
use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue},
    prelude::*,
};
use std::sync::{Arc, Mutex};

use super::Camera;
use crate::game::GameData;

pub struct Terrain {
    _data: Arc<Mutex<GameData>>,
    pub width: u16,
    pub height: u16,
    terrain_texture: Texture2D,
    mask_image: Image,
    mask_texture: Texture2D,
    material: Material,
    update_uniforms: bool,
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
        let image = Image::from_file_with_format(
            texture.texture.as_slice(),
            Some(macroquad::prelude::ImageFormat::Png),
        )?;
        let terrain_texture = Texture2D::from_image(&image);
        terrain_texture.set_filter(FilterMode::Nearest);

        let mask_image = Image::gen_image_color(width, height, WHITE);
        let mask_texture = Texture2D::from_image(&mask_image);
        mask_texture.set_filter(FilterMode::Nearest);

        let data_clone = data.clone();
        let shader = data_clone
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

        Ok(Self {
            _data: data,
            width,
            height,
            terrain_texture,
            mask_image,
            mask_texture,
            material,
            update_uniforms: true,
        })
    }

    pub fn destruct(&mut self, loc_x: u32, loc_y: u32, radius: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as i32 - loc_x as i32;
                let dy = y as i32 - loc_y as i32;
                if dx * dx + dy * dy <= radius as i32 * radius as i32 {
                    self.mask_image.set_pixel(x.into(), y.into(), BLACK);
                }
            }
        }
        self.mask_texture.update(&self.mask_image);
        self.update_uniforms = true;
    }

    pub fn update(&mut self) {
        if self.update_uniforms {
            self.update_uniforms = false;
            self.material
                .set_texture("tex", self.terrain_texture.clone());
            self.material.set_texture("mask", self.mask_texture.clone());
            self.material.set_uniform("offset", Vec2::ZERO);
        }
    }

    pub fn draw(&self, camera: &Camera) {
        gl_use_material(&self.material);
        draw_texture_ex(
            &self.terrain_texture,
            -0.5 - camera.target.x * camera.zoom,
            0.5 + camera.target.y * camera.zoom / screen_height() * screen_width(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(1.0 * 1.5, -1.0 * 1.5)),
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}
