use bevy_ecs::{
    prelude::{Commands, Schedule, World},
    schedule::IntoScheduleConfigs,
    system::{Query, Res},
};
use deferred::Renderer;
use nalgebra::Vector2;
use winit::keyboard::{Key, NamedKey};

use super::Scene;
use crate::{
    data::GameData,
    ecs::{
        init_camera, process_ecs, update_cameras, CameraTarget, Data, Inputs, RendererRes, Texture,
        Transform,
    },
};
use utils::prelude::*;

fn init_objects(mut commands: Commands) {
    for i in 0..16 {
        let id = commands
            .spawn((
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
            ))
            .id();

        if i == 7 {
            commands.entity(id).insert(CameraTarget {});
        }
    }
}

fn update_objects(mut query: Query<(&mut Transform,)>, input: Res<Inputs>) {
    let input = input.0.lock_panic();
    for (mut transform,) in query.iter_mut() {
        if input.is_key_down(Key::Named(NamedKey::ArrowUp)) {
            transform.position.y -= 2.0;
        }
        if input.is_key_down(Key::Named(NamedKey::ArrowDown)) {
            transform.position.y += 2.0;
        }

        if input.is_key_down(Key::Named(NamedKey::ArrowLeft)) {
            transform.position.x -= 2.0;
        }
        if input.is_key_down(Key::Named(NamedKey::ArrowRight)) {
            transform.position.x += 2.0;
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
        init.add_systems((init_camera, init_objects));
        init.run(&mut world);

        let mut update = Schedule::default();
        update.add_systems((update_objects, update_cameras).chain());

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
