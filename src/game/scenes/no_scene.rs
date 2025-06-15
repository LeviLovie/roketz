use super::Scene;
use crate::game::GameData;
use std::sync::{Arc, Mutex};
use tracing::debug;

pub struct NoScene;

impl Scene for NoScene {
    fn create(_data: Arc<Mutex<GameData>>) -> Self {
        debug!("Scene NoScene created");
        Self
    }

    fn should_transfer(&self) -> Option<String> {
        None
    }

    fn render(&self) {}
}
