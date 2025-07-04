use rdss::Loader;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::{config::Config, scenes::BattleSettings};
use sound::SoundEngine;

pub struct GameData {
    pub config: Rc<RefCell<Config>>,
    pub assets: Loader,
    pub sound_engine: Arc<Mutex<SoundEngine>>,
    pub debug: bool,
    pub battle_settings: BattleSettings,
}
