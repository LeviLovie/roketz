use bevy_ecs::prelude::Resource;

use deferred::Renderer;
use utils::prelude::*;

#[derive(Resource)]
pub struct RendererRes(pub MArc<Renderer>);
