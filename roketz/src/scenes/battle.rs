use bevy_ecs::prelude::{Commands, Schedule, World};
use deferred::Renderer;
use nalgebra::Vector2;

use super::Scene;
use crate::{
    data::GameData,
    ecs::{process_ecs, RendererRes, Texture, Transform},
};
use utils::prelude::*;

fn init_objects(mut commands: Commands) {
    for i in 0..16 {
        commands.spawn((
            Transform {
                position: Vector2::new(10.0, 10.0 + i as f32 * 50.0),
                scale: Vector2::new(3.0, 3.0),
                rotation: 180.0,
                layer: i % 4,
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

pub struct BattleScene {
    world: World,
    update: Schedule,
}

impl Scene for BattleScene {
    fn name(&self) -> String {
        "BattleScene".to_string()
    }

    #[instrument(skip_all)]
    fn create(_data: MArc<GameData>) -> Result<Self> {
        let mut world = World::new();

        let mut init = Schedule::default();
        init.add_systems(init_objects);
        init.run(&mut world);

        let update = Schedule::default();

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

    // fn render(&mut self) -> Objects {
    //     vec![
    //         (
    //             -25.0,
    //             vec![Object {
    //                 pos: [350.0, 100.0].into(),
    //                 size: [100.0, 50.0].into(),
    //                 color: [1.0, 0.0, 0.0, 1.0].into(),
    //             }],
    //         ),
    //         (
    //             -10.0,
    //             vec![Object {
    //                 pos: [350.0, 150.0],
    //                 size: [100.0, 50.0],
    //                 color: [0.0, 1.0, 0.0, 1.0],
    //             }],
    //         ),
    //         (
    //             0.0,
    //             vec![Object {
    //                 pos: [350.0, 200.0],
    //                 size: [100.0, 50.0],
    //                 color: [1.0, 1.0, 1.0, 1.0],
    //             }],
    //         ),
    //         (
    //             10.0,
    //             vec![Object {
    //                 pos: [350.0, 250.0],
    //                 size: [100.0, 50.0],
    //                 color: [0.0, 0.0, 1.0, 1.0],
    //             }],
    //         ),
    //     ]
    // }
}
