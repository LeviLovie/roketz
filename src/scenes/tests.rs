use anyhow::{Context, Result};
use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::{
    components::Transform,
    game::{GameData, Scene},
    systems::update_transforms,
};

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

        let mut world = World::new();
        let mut schedule = Schedule::default();

        world.spawn((Transform::from_pos(vec2(25.0, 25.0)),));

        schedule.add_systems((update_transforms,));

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
