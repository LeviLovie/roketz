use std::{cell::RefCell, rc::Rc};

use crate::{config::Config, scenes::BattleSettings};
use rasset::prelude::Registry;

pub struct GameData {
    pub config: Rc<RefCell<Config>>,
    pub assets: Registry,
    pub debug: bool,
    pub battle_settings: BattleSettings,
}
