use crate::config::Config;
use rasset::prelude::Registry;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DebugState {
    pub enabled: bool,
    pub v_player: bool,
    pub v_terrain: bool,
    pub ol_bvh: bool,
    pub ol_physics: bool,
}

pub struct GameData {
    pub config: Config,
    pub assets: Registry,
    pub debug: DebugState,
}
