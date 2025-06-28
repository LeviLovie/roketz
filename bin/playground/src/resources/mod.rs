use bevy_ecs::prelude::*;
use macroquad::prelude::*;

#[derive(Resource)]
pub struct Selection {
    pub entity: Option<Entity>,
    pub changed: bool,
}

impl Default for Selection {
    fn default() -> Self {
        Selection {
            entity: None,
            changed: false,
        }
    }
}
