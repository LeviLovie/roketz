use anyhow::Result;
use std::sync::{Arc, Mutex};

use crate::game::GameData;

pub trait Scene: Send + Sync + 'static {
    fn scene(self) -> Box<dyn Scene>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    fn create(data: Arc<Mutex<GameData>>) -> Result<Self>
    where
        Self: Sized;

    fn name(&self) -> &str;

    fn should_transfer(&self) -> Option<String> {
        None
    }

    fn update(&mut self) {}

    fn render(&self) {}

    fn destroy(&mut self) {}
}
