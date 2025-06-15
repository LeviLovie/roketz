use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tracing::{debug, trace, warn};

use super::{NoScene, Scene};
use crate::game::GameData;

type Scenes = HashMap<String, Box<dyn Scene>>;

#[allow(unused)]
pub struct SceneManager {
    data: Arc<Mutex<GameData>>,
    scenes: Arc<Mutex<Scenes>>,
    current: String,
}

impl SceneManager {
    pub fn new(data: Arc<Mutex<GameData>>) -> Self {
        let mut scenes = HashMap::new();
        let no_scene = NoScene::create(data.clone());
        scenes.insert(
            no_scene.name().to_string(),
            Box::new(NoScene::create(data.clone())).scene(),
        );

        debug!("SceneManager created");
        Self {
            data,
            scenes: Arc::new(Mutex::new(scenes)),
            current: no_scene.name().to_string(),
        }
    }

    pub fn add_scene<S>(&mut self, scene: S)
    where
        S: Scene + 'static,
    {
        let name = scene.name().to_string();
        let mut scenes = self.scenes.lock().unwrap();
        if scenes.contains_key(&name) {
            panic!("Scene '{}' already exists", name);
        }
        scenes.insert(name.clone(), Box::new(scene).scene());
        debug!(name = ?name, "Scene added");
    }

    pub fn update(&mut self) -> Result<()> {
        let next_scene = self.with_current_scene_mut(|scene| {
            scene.update();
            scene.should_transfer()
        });

        if let Some(next) = next_scene {
            self.transfer_to(next)
                .context(format!("Updating {}", self.current))?;
        }
        Ok(())
    }

    pub fn render(&self) {
        self.with_current_scene(|scene| {
            scene.render();
        });
    }

    pub fn destroy(&mut self) {
        for scene in self.scenes.lock().unwrap().values_mut() {
            trace!(name = ?scene.name(), "Destroying scene");
            scene.destroy();
        }
        debug!("All scenes destroyed");
    }

    pub fn transfer_to(&mut self, next_scene: String) -> Result<()> {
        debug!(scene = ?next_scene, "Transferring to scene");
        let scenes = self.scenes.lock().unwrap();
        if !scenes.contains_key(&next_scene) {
            warn!(scene = ?next_scene, "Scene not found, transferring to 'no_scene'");
            let no_scene = scenes.get("no_scene").ok_or_else(|| {
                anyhow::anyhow!("No scene was found, and `no_scene` is not initialized; crashing")
                    .context("Transferring to 'no_scene'")
                    .context(format!("Transferring to {}", next_scene))
            })?;
            self.current = no_scene.name().to_string();
        }
        self.current = next_scene;
        Ok(())
    }

    fn with_current_scene<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Box<dyn Scene>) -> R,
    {
        let scenes = self.scenes.lock().unwrap();
        let scene = scenes
            .get(&self.current)
            .unwrap_or_else(|| panic!("Scene '{}' not found", self.current));
        f(scene)
    }

    fn with_current_scene_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Box<dyn Scene>) -> R,
    {
        let mut scenes = self.scenes.lock().unwrap();
        let scene = scenes
            .get_mut(&self.current)
            .unwrap_or_else(|| panic!("Scene '{}' not found", self.current));
        f(scene)
    }
}
