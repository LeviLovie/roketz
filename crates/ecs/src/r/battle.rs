use bevy_ecs::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BattleType {
    Single,
    MultiTopBottom,
    MultiLeftRight,
}

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct BattleSettings {
    pub ty: BattleType,
    pub map: Option<String>,
}

impl Default for BattleSettings {
    fn default() -> Self {
        Self {
            ty: BattleType::Single,
            map: None,
        }
    }
}
