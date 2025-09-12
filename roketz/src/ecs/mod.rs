pub mod renderer;
pub mod texture;
pub mod transform;

#[allow(unused_imports)]
pub use renderer::*;
#[allow(unused_imports)]
pub use texture::*;
#[allow(unused_imports)]
pub use transform::*;

use bevy_ecs::prelude::*;

pub fn process_ecs() -> Schedule {
    let mut schedule = Schedule::default();

    schedule.add_systems((load_textures, clear_objects, transfer_objects).chain());

    schedule
}
