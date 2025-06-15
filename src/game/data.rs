use crate::config::Config;
use rasset::prelude::Registry;

#[allow(unused)]
pub struct GameData {
    pub config: Config,
    pub assets: Registry,
    pub is_debug: bool,
}
