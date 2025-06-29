mod battle;
mod menu;

use std::{cell::RefCell, rc::Rc};

pub use battle::{Battle, BattleSettings, BattleType};
pub use menu::Menu;

use anyhow::Result;

use crate::game::{GameData, Scene, SceneManager};

pub fn register(manager: &mut SceneManager, data: Rc<RefCell<GameData>>) -> Result<()> {
    manager.add_scene(battle::Battle::create(data.clone())?)?;
    manager.add_scene(menu::Menu::create(data.clone())?)?;

    manager.transfer_to("Menu".to_string())?;
    Ok(())
}
