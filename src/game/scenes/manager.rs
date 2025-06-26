use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tracing::{debug, info, trace, warn};

use super::{NoScene, Scene};
use crate::game::GameData;

type Scenes = HashMap<String, Box<dyn Scene>>;

#[allow(unused)]
pub struct SceneManager {
    data: Option<Arc<Mutex<GameData>>>,
    scenes: Arc<Mutex<Scenes>>,
    current: String,
    quit: bool,
}

impl SceneManager {
    pub fn new(data: Option<Arc<Mutex<GameData>>>) -> Result<Self> {
        let mut manager = Self {
            data: data.clone(),
            scenes: Arc::new(Mutex::new(HashMap::new())),
            current: "no_scene".to_string(),
            quit: false,
        };

        manager
            .add_scene(NoScene::create(data.clone())?)
            .context("Failed to add 'no_scene'")?;

        info!("SceneManager created");
        Ok(manager)
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    fn get_scenes(&self) -> Result<std::sync::MutexGuard<Scenes>> {
        self.scenes
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock scenes: {}", e))
    }

    fn get_scenes_mut(&mut self) -> Result<std::sync::MutexGuard<Scenes>> {
        self.scenes
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock scenes: {}", e))
    }

    pub fn add_scene<S>(&mut self, scene: S) -> Result<()>
    where
        S: Scene + 'static,
    {
        let name = scene.name().to_string();
        let mut scenes = self.get_scenes_mut()?;
        if scenes.contains_key(&name) {
            Err(anyhow::anyhow!("Scene with name '{}' already exists", name)
                .context(format!("Adding scene {}", name)))?;
        }
        scenes.insert(name.clone(), Box::new(scene).scene());
        trace!(name = ?name, "Scene added");
        Ok(())
    }

    pub fn remove_scene(&mut self, name: &str) -> Result<()> {
        let mut scenes = self.get_scenes_mut()?;
        if scenes.remove(name).is_none() {
            Err(anyhow::anyhow!("Scene '{}' not found", name)
                .context(format!("Removing scene {}", name)))?;
        } else {
            trace!(name = ?name, "Scene removed");
        }
        Ok(())
    }

    pub fn current_scene(&self) -> Result<String> {
        self.with_current_scene(|scene| Ok(scene.name().to_string()))?
    }

    pub fn update(&mut self) -> Result<()> {
        let next_scene = self.with_current_scene_mut(|scene| {
            scene.update();
            scene.should_transfer()
        })?;

        if let Some(next) = next_scene {
            self.transfer_to(next)
                .context(format!("Updating {}", self.current))?;
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        self.with_current_scene_mut(|scene| {
            scene.render();
        })
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        self.with_current_scene_mut(|scene| {
            scene.ui(ctx);
        })
        .unwrap_or_else(|e| {
            warn!(error = ?e, "Failed to render UI for current scene");
        });
    }

    pub fn destroy(&mut self) {
        let mut scenes = self.get_scenes_mut().unwrap();
        for scene in scenes.values_mut() {
            trace!(name = ?scene.name(), "Scene destroyed");
            scene.destroy();
        }
        debug!("SceneManager destroyed");
    }

    pub fn transfer_to(&mut self, next_scene: String) -> Result<()> {
        if next_scene == "__quit" {
            self.quit = true;
            debug!("Quit scene recived");
            return Ok(());
        }

        {
            debug!(scene = ?next_scene, "Transferring to scene");
            self.current = next_scene.clone();
            let mut new_current = self.current.clone();
            {
                let scenes = self.get_scenes()?;
                if !scenes.contains_key(&next_scene) {
                    warn!(scene = ?next_scene, "Scene not found, transferring to 'no_scene'");
                    let no_scene = scenes.get("no_scene").ok_or_else(|| {
                        anyhow::anyhow!(
                            "No scene was found, and `no_scene` is not initialized; crashing"
                        )
                        .context("Transferring to 'no_scene'")
                        .context(format!("Transferring to {}", next_scene))
                    })?;
                    new_current = no_scene.name().to_string();
                }
            }
            self.current = new_current;
        }

        self.with_current_scene_mut(|scene| -> Result<()> {
            scene
                .reload()
                .context(format!("Reloading scene {}", next_scene))?;
            trace!(name = ?scene.name(), "Scene reloaded");
            Ok(())
        })??;

        Ok(())
    }

    fn with_current_scene<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Box<dyn Scene>) -> R,
    {
        let scenes = self.get_scenes()?;
        let scene = scenes.get(&self.current).ok_or_else(|| {
            anyhow::anyhow!("Scene '{}' not found", self.current).context("Accessing current scene")
        })?;
        Ok(f(scene))
    }

    fn with_current_scene_mut<F, R>(&mut self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Box<dyn Scene>) -> R,
    {
        let current_scene = self.current.clone();
        let mut scenes = self.get_scenes_mut()?;
        let scene = scenes.get_mut(&current_scene).ok_or_else(|| {
            anyhow::anyhow!("Scene '{}' not found", current_scene)
                .context("Accessing current scene mutably")
        })?;
        Ok(f(scene))
    }
}
