use anyhow::{Context, Result};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::{
    game::{GameData, Scene},
    wrappers::{Camera, Player, Terrain},
};

#[allow(unused)]
pub struct Battle {
    data: Arc<Mutex<GameData>>,
    player: Player,
    camera: Camera,
    terrain: Terrain,
}

impl Scene for Battle {
    fn create(data: Arc<Mutex<GameData>>) -> Result<Self> {
        let terrain_data = data
            .lock()
            .unwrap()
            .assets
            .get_asset::<assets::Terrain>("TestTerrain")
            .context("Failed to get terrain texture")?
            .clone();

        let terrain =
            Terrain::new(data.clone(), &terrain_data).context("Failed to create terrain")?;
        let camera = Camera::new();

        let mut player = Player::new(
            data.clone(),
            vec2(
                terrain_data.player_start_x as f32,
                terrain_data.player_start_y as f32,
            ),
        );
        player.gravity = 50.0;

        Ok(Self {
            data: data.clone(),
            player,
            camera,
            terrain,
        })
    }

    fn name(&self) -> &str {
        "Battle"
    }

    fn update(&mut self) {
        if is_key_pressed(KeyCode::T) {
            let player_pos = self.player.get_position();
            self.terrain.destruct(
                player_pos.x as u32,
                player_pos.y as u32,
                self.terrain.destruction_radius,
            );
        }

        self.player.update(&mut self.terrain);
        self.camera.target = self.player.get_position();
        self.camera.update();
        self.terrain.update();
    }

    fn render(&self) {
        clear_background(DARKGRAY);

        self.terrain.draw(&self.camera);
        self.player.draw();

        set_default_camera();

        self.player.ui();
    }
}
