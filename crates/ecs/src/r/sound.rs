use bevy_ecs::prelude::*;
use std::sync::{Arc, Mutex};

use sound::SoundEngine;

#[derive(Resource)]
pub struct Sound(pub Arc<Mutex<SoundEngine>>);

impl Sound {
    pub fn borrow(&self) -> std::sync::MutexGuard<'_, SoundEngine> {
        self.0.lock().unwrap()
    }
}
