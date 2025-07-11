use bevy_ecs::prelude::*;

#[derive(Resource, Debug)]
pub struct DT(pub f32);

pub fn init_dt(mut commands: Commands) {
    commands.insert_resource(DT(0.0));
}
