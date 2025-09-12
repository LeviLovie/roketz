use bevy_ecs::prelude::Resource;
use utils::marc::MArc;

#[derive(Resource)]
pub struct Inputs(pub MArc<crate::inputs::Inputs>);
