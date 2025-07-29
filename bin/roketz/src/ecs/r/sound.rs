use bevy_ecs::prelude::*;
use helpers::error::HandleError;

#[cfg(feature = "fmod")]
use sound::SoundEngine;
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct Sound(pub Arc<Mutex<SoundEngine>>);

impl Sound {
    pub fn new(engine: Arc<Mutex<SoundEngine>>) -> Self {
        Self(engine)
    }

    pub fn borrow(&self) -> std::sync::MutexGuard<'_, SoundEngine> {
        self.0.lock().handle("Failed to lock SoundEngine mutex")
    }
}

#[cfg(not(feature = "fmod"))]
pub struct SoundEngine {}

#[cfg(not(feature = "fmod"))]
impl SoundEngine {
    pub fn new(_bank: &str, _adds: Vec<&str>) -> Self {
        SoundEngine {}
    }

    pub fn list(&self) -> Result<()> {
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn play(&self, _event_path: &str) -> Result<()> {
        Ok(())
    }

    pub fn play_looping(&mut self, _event_path: &str) -> Result<()> {
        Ok(())
    }

    pub fn stop_looping(&mut self, _event_path: &str) -> Result<()> {
        Ok(())
    }

    pub fn set_parameter(&mut self, _event_path: &str, _param: &str, _value: f32) -> Result<()> {
        Ok(())
    }
}
