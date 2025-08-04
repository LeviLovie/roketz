use rdss::Loader;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::{config::Config, ecs::r::BattleSettings, sprites::Sprites};

#[cfg(not(feature = "fmod"))]
use crate::ecs::r::SoundEngine;
#[cfg(feature = "fmod")]
use sound::SoundEngine;

pub struct GameData {
    pub config: Rc<RefCell<Config>>,
    pub assets: Arc<Mutex<Loader>>,
    pub sprites: Arc<Mutex<Sprites>>,
    pub sound: Arc<Mutex<SoundEngine>>,
    pub debug: bool,
    pub battle_settings: BattleSettings,
}
