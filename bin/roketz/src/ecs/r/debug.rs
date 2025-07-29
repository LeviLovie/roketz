use bevy_ecs::prelude::*;

#[derive(Resource, Default)]
pub struct Debug {
    pub o_physics: bool,
    pub o_bvh: bool,
    pub p_dt: bool,
}

pub fn init_debug(mut commands: Commands) {
    commands.insert_resource(Debug::default());
}
