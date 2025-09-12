mod player;
mod terrain;

use bevy_ecs::{
    prelude::{Commands, Schedule, World},
    schedule::IntoScheduleConfigs,
};
use deferred::Renderer;
use nalgebra::Vector2;

use super::Scene;
use crate::{
    data::GameData,
    ecs::{
        init_camera, process_ecs, update_cameras, Data, Inputs, RendererRes, Texture, Transform,
    },
    scenes::battle::terrain::spawn_terrain,
};
use player::{spawn_player, update_players};
use utils::prelude::*;

fn init_objects(mut commands: Commands) {
    for x in 0..4 {
        for y in 0..6 {
            commands.spawn((
                Transform {
                    position: Vector2::new(-300.0 + x as f32 * 150.0, -450.0 + y as f32 * 150.0),
                    scale: Vector2::new(3.0, 3.0),
                    rotation: 0.0,
                    layer: 3,
                },
                Texture {
                    handle: None,
                    path: "rocket.png".to_string(),
                    width: 32,
                    height: 32,
                },
            ));
        }
    }
}

pub struct BattleScene {
    world: World,
    update: Schedule,
}

impl Scene for BattleScene {
    fn name(&self) -> String {
        "Battle".to_string()
    }

    #[instrument(skip_all)]
    fn create(data: MArc<GameData>) -> Result<Self> {
        let mut world = World::new();
        world.insert_resource(Data(data.clone()));
        world.insert_resource(Inputs(data.lock_panic().inputs.clone()));

        let mut init = Schedule::default();
        init.add_systems((init_camera, init_objects, spawn_player, spawn_terrain));
        init.run(&mut world);

        let mut update = Schedule::default();
        update.add_systems((update_players, update_cameras).chain());

        Ok(Self { world, update })
    }

    fn update(&mut self) {
        self.update.run(&mut self.world);
    }

    fn render(&mut self, renderer: MArc<Renderer>) {
        self.world.insert_resource(RendererRes(renderer));
        process_ecs().run(&mut self.world);
        self.world.remove_resource::<RendererRes>();
    }
}
