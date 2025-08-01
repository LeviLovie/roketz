use bevy_ecs::prelude::*;

use crate::{camera::CameraType, ecs::cs::Transform};

#[derive(Component)]
pub struct CameraTarget(pub CameraType);
