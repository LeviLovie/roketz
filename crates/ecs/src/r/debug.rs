use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct Debug {
    pub o_physics: bool,
    pub o_bvh: bool,
    pub p_dt: bool,
}

impl Default for Debug {
    fn default() -> Self {
        Debug {
            o_physics: false,
            o_bvh: true,
            p_dt: true,
        }
    }
}

pub fn init_debug(mut commands: Commands) {
    commands.insert_resource(Debug::default());
}
