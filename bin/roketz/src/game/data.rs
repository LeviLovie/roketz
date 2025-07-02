use std::{cell::RefCell, rc::Rc};

use crate::{config::Config, scenes::BattleSettings};
use rdss::Loader;

pub struct GameData {
    pub config: Rc<RefCell<Config>>,
    pub assets: Loader,
    pub debug: bool,
    pub battle_settings: BattleSettings,
}
