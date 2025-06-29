use anyhow::Result;
use std::{cell::RefCell, rc::Rc};

use crate::game::GameData;

pub trait Scene: 'static {
    fn scene(self) -> Box<dyn Scene>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    fn create(data: Rc<RefCell<GameData>>) -> Result<Self>
    where
        Self: Sized;

    fn reload(&mut self) -> Result<()> {
        Ok(())
    }

    fn name(&self) -> &str;

    fn should_transfer(&self) -> Option<String> {
        None
    }

    fn update(&mut self) {}

    fn render(&mut self) {}

    fn ui(&mut self, _ctx: &egui::Context) -> Result<()> {
        Ok(())
    }

    fn destroy(&mut self) {}
}
