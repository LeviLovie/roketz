use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct RenderConfig {
    pub filled_in: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        RenderConfig { filled_in: true }
    }
}
