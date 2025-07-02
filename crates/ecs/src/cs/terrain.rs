use anyhow::{Context, Result};
use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use rapier2d::prelude::*;
use rdss::Loader;
use tracing::{debug, trace, warn};

use crate::{
    cs::{RigidCollider, Transform},
    get_map,
    r::PhysicsWorld,
};
use bvh::BVH;

#[derive(Component)]
pub struct Terrain {
    pub width: u16,
    pub height: u16,
    pub bvh: BVH,
    pub spawns: Vec<Vec2>,
    pub terrain_image: Image,
    pub terrain_texture: Texture2D,
    pub terrain_update: bool,
}

impl Terrain {
    pub fn new(assets: &mut Loader, id: String) -> Result<Self> {
        let map = get_map(assets, &id).context("Failed to get map")?;
        let texture = assets
            .read_raw(&format!("maps/{}/{}", id, map.texture))
            .context("Failed to read terrain texture")?;
        let map_file = assets
            .read_raw(&format!("maps/{}/{}", id, map.map))
            .context("Failed to read terrain map")?;

        let spawns = map
            .spawns
            .spawns
            .iter()
            .map(|spawn| vec2(spawn.x as f32, spawn.y as f32))
            .collect::<Vec<Vec2>>();

        let mut terrain_image =
            Image::from_file_with_format(&texture, Some(macroquad::prelude::ImageFormat::Png))?;
        let terrain_texture = Texture2D::from_image(&terrain_image);
        terrain_texture.set_filter(FilterMode::Nearest);

        let terrain_map =
            Image::from_file_with_format(&map_file, Some(macroquad::prelude::ImageFormat::Png))?;

        let (width, height) = Self::check_terrain_texture_sizes(&terrain_texture, &terrain_map);
        trace!(?width, ?height, "Creating terrain");

        let mut bvh = BVH::new(width as u32, height as u32, 8);

        for y in 0..height {
            for x in 0..width {
                let pixel = terrain_map.get_pixel(x as u32, y as u32);
                if pixel.r <= 0.1 && pixel.g <= 0.1 && pixel.b <= 0.1 {
                    bvh.cut_point(vec2(x as f32, y as f32));
                    terrain_image.set_pixel(x as u32, y as u32, Color::new(0.0, 0.0, 0.0, 0.0));
                }
            }
        }

        debug!("Terrain created");
        Ok(Self {
            width,
            height,
            bvh,
            spawns,

            terrain_image,
            terrain_texture,
            terrain_update: true,
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

    pub fn destruct(&mut self, loc_x: u32, loc_y: u32, radius: u32) -> Result<()> {
        self.bvh
            .cut_circle(vec2(loc_x as f32, loc_y as f32), radius as f32)
            .context("Failed to cut circle in terrain")?;

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

        Ok(())
    }

    pub fn destruct_point(&mut self, loc: Vec2) {
        self.bvh.cut_point(loc);
        self.terrain_image
            .set_pixel(loc.x as u32, loc.y as u32, Color::new(0.0, 0.0, 0.0, 0.0));
        self.terrain_update = true;
    }
}

#[derive(Component)]
pub struct TerrainCollider {}

pub fn update_terrain(
    mut commands: Commands,
    mut terrain: Query<&mut Terrain>,
    physics: ResMut<PhysicsWorld>,
    mut terrain_colliders: Query<(Entity, &mut RigidCollider), With<TerrainCollider>>,
) {
    let mut physics: Mut<PhysicsWorld> = physics.into();

    if let Ok(mut terrain) = terrain.single_mut()
        && terrain.terrain_update
    {
        terrain.terrain_update = false;
        terrain.terrain_texture.update(&terrain.terrain_image);

        for (entity, mut collider) in terrain_colliders.iter_mut() {
            collider.despawn(&mut physics);
            commands.entity(entity).despawn();
        }

        for (_, bounds) in terrain.bvh.get_nodes() {
            let pos = bounds.center();
            commands.spawn((
                Transform::from_pos(pos),
                RigidCollider::fixed(
                    &mut physics,
                    ColliderBuilder::cuboid(bounds.width() / 2.0, bounds.height() / 2.0).build(),
                    vector![pos.x, pos.y],
                    0.0,
                ),
                TerrainCollider {},
            ));
        }
    }
}

pub fn draw_terrain(query: Query<&Terrain>) {
    if let Ok(terrain) = query.single() {
        draw_texture(&terrain.terrain_texture, 0.0, 0.0, WHITE);
    }
}
