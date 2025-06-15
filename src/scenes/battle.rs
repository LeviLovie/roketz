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
        let terrain = Terrain::new(data.clone()).context("Failed to create terrain")?;

        let mut player = Player::new(data.clone());
        player.teleport(Vec2::new(-25.0, 0.0), -std::f32::consts::PI / 2.0);

        Ok(Self {
            data: data.clone(),
            player,
            camera: Camera::new(),
            terrain,
        })
    }

    fn name(&self) -> &str {
        "Battle"
    }

    fn update(&mut self) {
        self.player.update();
        self.terrain.update();
        self.camera.update();
    }

    fn render(&self) {
        clear_background(DARKGRAY);

        self.terrain.draw(&self.camera);
        self.player.draw();
    }
}
