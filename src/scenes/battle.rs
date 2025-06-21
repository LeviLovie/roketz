use anyhow::{Context, Result};
use macroquad::prelude::*;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::{
    game::{GameData, Scene},
    rhai,
    wrappers::{Camera, Player, Terrain},
};

pub struct Battle {
    data: Arc<Mutex<GameData>>,
    rhai_engine: Rc<RefCell<rhai::Engine>>,
    player: Player,
    camera: Camera,
    terrain: Terrain,
}

impl Scene for Battle {
    fn create(data: Arc<Mutex<GameData>>) -> Result<Self> {
        let scripts_path = data.lock().unwrap().config.scripts.path.clone();
        let rhai_engine = Rc::new(RefCell::new(rhai::Engine::new(scripts_path)));
        {
            let mut engine = rhai_engine.borrow_mut();
            engine.load("test");
            engine.reload();
            engine.init();
        }

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
            rhai_engine,
            player,
            camera,
            terrain,
        })
    }

    fn name(&self) -> &str {
        "Battle"
    }

    fn update(&mut self) {
        if is_key_pressed(KeyCode::F1) {
            self.rhai_engine.borrow_mut().reload();
        }
        self.rhai_engine.borrow_mut().update();

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

        self.rhai_engine.borrow_mut().render();
        self.terrain.draw(&self.camera);
        self.player.draw();

        set_default_camera();

        self.rhai_engine.borrow_mut().ui();
        self.player.ui();
    }
}
