use crate::config::Config;
use rasset::prelude::Registry;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum DebugMode {
    #[default]
    Disabled,

    PlayerPhysics,
    BVH,
}

pub struct GameData {
    pub config: Config,
    pub assets: Registry,
    pub debug: DebugMode,
}
