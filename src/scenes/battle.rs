use anyhow::{Context, Result};
use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::{
    camera::Camera,
    ecs::{
        cs::{
            draw_players, draw_terrain, update_physics, update_players, update_terrain, Physics,
            Player, Terrain, Transform,
        },
        res::{Gravity, DT},
    },
    game::{GameData, Scene},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BattleType {
    Single,
    MultiTopBottom,
    MultiLeftRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleSettings {
    pub ty: BattleType,
}

impl Default for BattleSettings {
    fn default() -> Self {
        Self {
            ty: BattleType::Single,
        }
    }
}

pub struct Battle {
    _data: Arc<Mutex<GameData>>,
    world: World,
    update: Schedule,
    draw: Schedule,
    cameras: Vec<Camera>,
}

impl Scene for Battle {
    fn name(&self) -> &str {
        "Battle"
    }

    fn create(data: Option<Arc<Mutex<GameData>>>) -> Result<Self> {
        let data = data.context("Battle scene requires GameData")?.clone();
        let terrain_data = data
            .lock()
            .unwrap()
            .assets
            .get_asset::<assets::Terrain>("TestTerrain")
            .context("Failed to get terrain texture")?
            .clone();
        let player_spawn_point = vec2(
            terrain_data.player_one_x as f32,
            terrain_data.player_one_y as f32,
        );

        let mut cameras = Vec::new();

        let mut world = World::new();
        let mut update = Schedule::default();
        let mut draw = Schedule::default();

        {
            world.insert_resource(DT(0.0));
            world.insert_resource(Gravity(9.81));

            world.spawn((Terrain::new(&terrain_data)?,));

            let player_id = world
                .spawn((
                    Player::new(Color::from_rgba(66, 233, 245, 255)),
                    Transform::from_pos(player_spawn_point),
                    Physics::default(),
                ))
                .id();
            cameras.push(Camera::new(player_id));

            update.add_systems((update_terrain, update_players, update_physics).chain());

            draw.add_systems((draw_terrain, draw_players).chain());
        }

        Ok(Self {
            _data: data,
            world,
            update,
            draw,
            cameras,
        })
    }

    fn update(&mut self) {
        self.world.resource_mut::<DT>().0 = get_frame_time();

        self.update.run(&mut self.world);

        for camera in self.cameras.iter_mut() {
            camera.update(&mut self.world);
        }
    }

    fn render(&mut self) {
        clear_background(DARKGRAY);

        for camera in self.cameras.iter() {
            camera.set();
            self.draw.run(&mut self.world);
        }
    }
}
