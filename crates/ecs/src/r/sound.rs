use bevy_ecs::prelude::*;

#[cfg(feature = "fmod")]
use sound::SoundEngine;
#[cfg(feature = "fmod")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "fmod")]
#[derive(Resource)]
pub struct Sound(pub Arc<Mutex<SoundEngine>>);

#[cfg(feature = "fmod")]
impl Sound {
    pub fn borrow(&self) -> std::sync::MutexGuard<'_, SoundEngine> {
        self.0.lock().unwrap()
    }
}

#[cfg(not(feature = "fmod"))]
#[derive(Resource)]
pub struct Sound;

#[cfg(not(feature = "fmod"))]
impl Sound {
    pub fn borrow(&self) -> std::sync::MutexGuard<'_, ()> {
        panic!("Sound engine is not enabled. Compile with the 'fmod' feature.");
    }
}
