use bevy_ecs::prelude::*;

use crate::camera::CameraType;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BattleType {
    Single,
    MultiTopBottom,
    MultiLeftRight,
}

impl BattleType {
    pub fn cameras(&self) -> Vec<CameraType> {
        match self {
            BattleType::Single => vec![CameraType::Global],
            BattleType::MultiTopBottom => vec![CameraType::Top, CameraType::Bottom],
            BattleType::MultiLeftRight => vec![CameraType::Left, CameraType::Right],
        }
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct BattleSettings {
    pub ty: BattleType,
    pub map: Option<String>,
}
