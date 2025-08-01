use rdss::Loader;
use std::sync::{Arc, Mutex};

use crate::{
    ecs::r::BattleSettings, resolutions::Resolutions, settings::Settings, sprites::Sprites,
};

#[cfg(not(feature = "fmod"))]
use crate::ecs::r::SoundEngine;
#[cfg(feature = "fmod")]
use sound::SoundEngine;

pub struct GameData {
    pub settings: Arc<Mutex<Settings>>,
    pub assets: Arc<Mutex<Loader>>,
    pub sprites: Arc<Mutex<Sprites>>,
    pub resolutions: Resolutions,
    pub sound: Arc<Mutex<SoundEngine>>,
    pub debug: bool,
    pub battle_settings: BattleSettings,
}
