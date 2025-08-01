use bevy_ecs::prelude::*;
use helpers::error::HandleError;
use rdss::Loader;
use std::sync::{Arc, Mutex};

use crate::sprites::Sprites;

#[derive(Resource)]
pub struct Data {
    pub assets: Arc<Mutex<Loader>>,
    pub sprites: Arc<Mutex<Sprites>>,
}

impl Data {
    pub fn borrow_assets(&self) -> std::sync::MutexGuard<'_, Loader> {
        self.assets.lock().handle("Failed to lock assets loader")
    }

    pub fn borrow_sprites(&self) -> std::sync::MutexGuard<'_, Sprites> {
        self.sprites.lock().handle("Failed to lock sprites loader")
    }
}

pub fn add_data(world: &mut World, assets: Arc<Mutex<Loader>>, sprites: Arc<Mutex<Sprites>>) {
    world.insert_resource(Data { assets, sprites });
}
