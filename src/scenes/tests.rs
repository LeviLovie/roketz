use anyhow::{Context, Result};
use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::game::{GameData, Scene};

pub struct Tests {
    _data: Arc<Mutex<GameData>>,
    world: World,
    schedule: Schedule,
}

impl Scene for Tests {
    fn name(&self) -> &str {
        "Tests"
    }

    fn create(data: Option<Arc<Mutex<GameData>>>) -> Result<Self> {
        let data = data.context("Tests scene requires GameData")?.clone();

        let world = World::new();
        let schedule = Schedule::default();

        Ok(Self {
            _data: data,
            world,
            schedule,
        })
    }

    fn update(&mut self) {
        self.schedule.run(&mut self.world);
    }

    fn render(&self) {
        clear_background(DARKGRAY);
    }
}
