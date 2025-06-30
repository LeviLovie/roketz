mod battle;
mod menu;

pub use battle::{Battle, BattleSettings, BattleType, SCENE_BATTLE};
pub use menu::{Menu, SCENE_MENU};

use anyhow::Result;
use std::{cell::RefCell, rc::Rc};

use crate::game::{GameData, Scene, SceneManager};

pub const SCENE_QUIT: &str = "__quit";
pub const SCENE_NO: &str = "no_scene";

pub fn register(manager: &mut SceneManager, data: Rc<RefCell<GameData>>) -> Result<()> {
    manager.add_scene(battle::Battle::create(data.clone())?)?;
    manager.add_scene(menu::Menu::create(data.clone())?)?;

    manager.transfer_to(SCENE_MENU.to_string())?;
    Ok(())
}
