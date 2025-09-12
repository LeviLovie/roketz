pub mod camera;
pub mod data;
pub mod inputs;
pub mod renderer;
pub mod texture;
pub mod transform;

pub use camera::*;
pub use data::*;
pub use inputs::*;
pub use renderer::*;
pub use texture::*;
pub use transform::*;

use bevy_ecs::prelude::*;

pub fn process_ecs() -> Schedule {
    let mut schedule = Schedule::default();

    schedule.add_systems(
        (
            load_textures,
            clear_objects,
            transfer_objects,
            transfer_camera,
        )
            .chain(),
    );

    schedule
}
