use rdss::Loader;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::{config::Config, scenes::BattleSettings};

#[cfg(feature = "fmod")]
use sound::SoundEngine;

pub struct GameData {
    #[cfg(feature = "fmod")]
    pub sound_engine: Arc<Mutex<SoundEngine>>,
    #[cfg(not(feature = "fmod"))]
    pub sound_engine: Arc<Mutex<()>>,

    pub config: Rc<RefCell<Config>>,
    pub assets: Loader,
    pub debug: bool,
    pub battle_settings: BattleSettings,
}
