use bevy_ecs::prelude::*;

use crate::camera::CameraType;

#[derive(Component)]
pub struct CameraTarget(pub CameraType);
