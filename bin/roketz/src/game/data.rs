use rdss::Loader;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::config::Config;
use ecs::r::BattleSettings;

#[cfg(not(feature = "fmod"))]
use ecs::r::SoundEngine;
#[cfg(feature = "fmod")]
use sound::SoundEngine;

pub struct GameData {
    pub config: Rc<RefCell<Config>>,
    pub assets: Arc<Mutex<Loader>>,
    pub sound: Arc<Mutex<SoundEngine>>,
    pub debug: bool,
    pub battle_settings: BattleSettings,
}
