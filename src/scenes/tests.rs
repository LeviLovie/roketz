use anyhow::{Context, Result};
use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::{
    ecs::{
        cs::{draw_players, update_physics, update_players, Physics, Player, Transform},
        res::{Gravity, DT},
    },
    game::{GameData, Scene},
};

pub struct Tests {
    _data: Arc<Mutex<GameData>>,
    world: World,
    update: Schedule,
    draw: Schedule,
}

impl Scene for Tests {
    fn name(&self) -> &str {
        "Tests"
    }

    fn create(data: Option<Arc<Mutex<GameData>>>) -> Result<Self> {
        let data = data.context("Tests scene requires GameData")?.clone();

        let mut world = World::new();
        let mut update = Schedule::default();
        let mut draw = Schedule::default();

        world.insert_resource(DT(0.0));
        world.insert_resource(Gravity(9.81));

        world.spawn((
            Player::new(Color::from_rgba(66, 233, 245, 255)),
            Transform::from_pos(vec2(25.0, 25.0)),
            Physics::default(),
        ));

        update.add_systems((update_players, update_physics).chain());

        draw.add_systems((draw_players,));

        Ok(Self {
            _data: data,
            world,
            update,
            draw,
        })
    }

    fn update(&mut self) {
        self.world.resource_mut::<DT>().0 = get_frame_time();

        self.update.run(&mut self.world);
    }

    fn render(&mut self) {
        clear_background(DARKGRAY);
        self.draw.run(&mut self.world);
    }
}
