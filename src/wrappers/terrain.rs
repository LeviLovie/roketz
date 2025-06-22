use anyhow::Result;
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use tracing::{debug, trace, warn};

use crate::{bvh::BVH, game::GameData};
use assets::Terrain as TerrainData;

pub struct Terrain {
    data: Arc<Mutex<GameData>>,
    pub width: u16,
    pub height: u16,
    pub(super) bvh: BVH,
    pub kill_distance_x: u32,
    pub kill_distance_y: u32,

    terrain_image: Image,
    terrain_texture: Texture2D,
    terrain_update: bool,
    destructions: Vec<(u32, u32, u32)>,
}

impl Terrain {
    pub fn new(data: Arc<Mutex<GameData>>, terrain_data: &TerrainData) -> Result<Self> {
        let mut terrain_image = Image::from_file_with_format(
            terrain_data.texture.as_slice(),
            Some(macroquad::prelude::ImageFormat::Png),
        )?;
        let terrain_texture = Texture2D::from_image(&terrain_image);
        terrain_texture.set_filter(FilterMode::Nearest);

        let terrain_map = Image::from_file_with_format(
            terrain_data.map.as_slice(),
            Some(macroquad::prelude::ImageFormat::Png),
        )?;

        let (width, height) = Self::check_terrain_texture_sizes(&terrain_texture, &terrain_map);
        trace!(?width, ?height, "Creating terrain");

        let bvh_depth = data.lock().unwrap().config.physics.bvh_depth as usize;
        let mut bvh = BVH::new(width as u32, height as u32, bvh_depth);

        for y in 0..height {
            for x in 0..width {
                let pixel = terrain_map.get_pixel(x as u32, y as u32);
                if pixel.r <= 0.1 && pixel.g <= 0.1 && pixel.b <= 0.1 {
                    bvh.cut_point(vec2(x as f32, y as f32));
                    terrain_image.set_pixel(x as u32, y as u32, Color::new(0.0, 0.0, 0.0, 0.0));
                }
            }
        }

        let kill_distance_x = width as u32 / 2 + terrain_data.kill_distance as u32;
        let kill_distance_y = height as u32 / 2 + terrain_data.kill_distance as u32;

        debug!("Terrain crated");
        Ok(Self {
            data: data.clone(),
            width,
            height,
            bvh,
            kill_distance_x,
            kill_distance_y,

            terrain_image,
            terrain_texture,
            terrain_update: true,
            destructions: Vec::new(),
        })
    }

    fn check_terrain_texture_sizes(terrain_texture: &Texture2D, terrain_map: &Image) -> (u16, u16) {
        let terrain_texture_width = terrain_texture.width() as u16;
        let terrain_texture_height = terrain_texture.height() as u16;
        let terrain_map_texture_width = terrain_map.width;
        let terrain_map_texture_height = terrain_map.height;

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
                    self.terrain_image.set_pixel(
                        x.into(),
                        y.into(),
                        Color::new(0.0, 0.0, 0.0, 0.0),
                    );
                }
            }
        }
        self.terrain_update = true;

        self.destructions.push((loc_x, loc_y, radius));
    }

    pub fn update(&mut self) {
        if self.terrain_update {
            self.terrain_update = false;
            self.terrain_texture.update(&self.terrain_image);
            self.terrain_texture.set_filter(FilterMode::Nearest);
        }
    }

    pub fn draw(&self) {
        draw_texture(&self.terrain_texture, 0.0, 0.0, WHITE);

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
