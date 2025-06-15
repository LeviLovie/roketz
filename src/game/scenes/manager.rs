use std::sync::{Arc, Mutex};
use tracing::debug;

use super::Scene;
use crate::game::GameData;

pub struct NoScene;

impl Scene for NoScene {
    fn create(_data: Arc<Mutex<GameData>>) -> Self {
        debug!("Scene NoScene created");
        Self
    }
}

#[allow(unused)]
pub struct SceneManager {
    data: Arc<Mutex<GameData>>,
    scenes: Arc<Mutex<Vec<Box<dyn Scene>>>>,
    current: usize,
}

impl SceneManager {
    pub fn new(data: Arc<Mutex<GameData>>) -> Self {
        let scenes = Arc::new(Mutex::new(vec![
            Box::new(NoScene::create(data.clone())) as Box<dyn Scene>
        ]));
        let current = 0;

        debug!("SceneManager created");
        Self {
            data,
            scenes,
            current,
        }
    }
}
