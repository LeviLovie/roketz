use bevy_ecs::prelude::*;

use super::{Data, Transform};

#[derive(Component)]
pub struct Camera(pub deferred::Camera);

#[derive(Component)]
pub struct CameraTarget {}

pub fn init_camera(mut commands: Commands, data: Res<Data>) {
    commands.spawn((Camera(data.0.lock_panic().camera),));
}

pub fn update_cameras(
    mut cameras: Query<&mut Camera>,
    targets: Query<&Transform, With<CameraTarget>>,
) {
    for mut camera in cameras.iter_mut() {
        if let Some(target) = targets.iter().next() {
            camera.0.set_center(target.position);
        }
    }
}

pub fn transfer_camera(camera: Query<&Camera>, data: Res<Data>) {
    if let Some(camera) = camera.iter().next() {
        data.0.lock_panic().camera = camera.0;
    }
}
