use bevy_ecs::prelude::*;

#[derive(Resource, Default)]
pub struct Debug {
    pub o_physics: bool,
}
