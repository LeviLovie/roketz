use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::{
    game::{GameData, Scene},
    wrappers::{Camera, Player},
};

#[allow(unused)]
pub struct Battle {
    data: Arc<Mutex<GameData>>,
    player: Player,
    camera: Camera,
}

impl Scene for Battle {
    fn create(data: Arc<Mutex<GameData>>) -> Self {
        Self {
            data: data.clone(),
            player: Player::new(data.clone()),
            camera: Camera::new(),
        }
    }

    fn name(&self) -> &str {
        "Battle"
    }

    fn update(&mut self) {
        self.player.update();
        self.camera.update();
    }

    fn render(&self) {
        clear_background(LIGHTGRAY);

        self.player.draw();
    }
}
