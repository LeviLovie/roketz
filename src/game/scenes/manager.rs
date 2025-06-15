use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tracing::debug;

use super::{NoScene, Scene};
use crate::game::GameData;

type Scenes = HashMap<String, Box<dyn Scene>>;

#[allow(unused)]
pub struct SceneManager {
    data: Arc<Mutex<GameData>>,
    scenes: Arc<Mutex<Scenes>>,
    current: usize,
}

impl SceneManager {
    pub fn new(data: Arc<Mutex<GameData>>) -> Self {
        let mut scenes = HashMap::new();
        scenes.insert(
            "no_scene".to_string(),
            Box::new(NoScene::create(data.clone())).scene(),
        );
        let current = 0;

        debug!("SceneManager created");
        Self {
            data,
            scenes: Arc::new(Mutex::new(scenes)),
            current,
        }
    }
}
