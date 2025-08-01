pub use bevy_ecs::prelude::*;
pub use macroquad::prelude::*;

use crate::{
    camera::Camera,
    ecs::{
        cs::{CameraTarget, Transform},
        r::BattleSettings,
    },
};

#[derive(Resource, Clone)]
pub struct Cameras(pub Vec<Camera>);

impl Cameras {
    pub fn push(&mut self, camera: Camera) {
        self.0.push(camera);
    }
}

pub fn init_cameras(world: &mut World) {
    let settings = world.resource::<BattleSettings>();
    let camera_types = settings.ty.cameras();
    let cameras = camera_types
        .iter()
        .map(|camera_type| Camera::new(camera_type.clone()))
        .collect::<Vec<_>>();
    world.insert_resource(Cameras(cameras));
}

pub fn update_cameras(
    targets: Query<&Transform, With<CameraTarget>>,
    mut cameras: ResMut<Cameras>,
) {
    let mut targets_clone = Vec::new();
    for target in targets.iter() {
        targets_clone.push(target);
    }

    for (i, mut camera) in cameras.0.iter_mut().enumerate() {
        if i >= targets_clone.len() {
            return;
        }

        camera.update(targets_clone[i]);
    }
}
