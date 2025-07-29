use bevy_ecs::prelude::*;
use helpers::error::HandleError;
use rdss::Loader;
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct Assets(pub Arc<Mutex<Loader>>);

impl Assets {
    pub fn borrow(&self) -> std::sync::MutexGuard<'_, Loader> {
        self.0.lock().handle("Failed to lock assets loader")
    }
}

pub fn add_assets(world: &mut World, loader: Arc<Mutex<Loader>>) {
    world.insert_resource(Assets(loader));
}
