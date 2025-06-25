mod battle;
mod menu;

pub use battle::{Battle, BattleSettings, BattleType};
pub use menu::Menu;

use anyhow::Result;
use std::sync::{Arc, Mutex};

use crate::game::{GameData, Scene, SceneManager};

pub fn register(manager: &mut SceneManager, data: Arc<Mutex<GameData>>) -> Result<()> {
    manager.add_scene(battle::Battle::create(Some(data.clone()))?)?;
    manager.add_scene(menu::Menu::create(Some(data.clone()))?)?;

    manager.transfer_to("Battle".to_string())?;
    Ok(())
}
