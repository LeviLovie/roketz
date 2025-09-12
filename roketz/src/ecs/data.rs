use bevy_ecs::prelude::Resource;
use utils::prelude::*;

#[derive(Resource)]
pub struct Data(pub MArc<crate::data::GameData>);
