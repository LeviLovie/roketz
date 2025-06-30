use anyhow::{bail, Context, Result};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tracing::{debug, info, trace, warn};

use super::{NoScene, Scene};
use crate::{
    game::GameData,
    scenes::{SCENE_NO, SCENE_QUIT},
};

type Scenes = HashMap<String, Box<dyn Scene>>;

#[allow(unused)]
pub struct SceneManager {
    data: Rc<RefCell<GameData>>,
    scenes: Rc<RefCell<Scenes>>,
    current: String,
    quit: bool,
}

impl SceneManager {
    pub fn new(data: Rc<RefCell<GameData>>) -> Result<Self> {
        let mut manager = Self {
            data: data.clone(),
            scenes: Rc::new(RefCell::new(HashMap::new())),
            current: SCENE_NO.to_string(),
            quit: false,
        };

        manager
            .add_scene(NoScene::create(data.clone())?)
            .context(format!("Failed to add {SCENE_NO}"))?;

        info!("SceneManager created");
        Ok(manager)
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn add_scene<S>(&mut self, scene: S) -> Result<()>
    where
        S: Scene + 'static,
    {
        let name = scene.name().to_string();
        let mut scenes = self.scenes.borrow_mut();
        if scenes.contains_key(&name) {
            bail!("Scene with name '{}' already exists", name);
        }
        scenes.insert(name.clone(), Box::new(scene).scene());
        trace!(name = ?name, "Scene added");
        Ok(())
    }

    pub fn remove_scene(&mut self, name: &str) -> Result<()> {
        let mut scenes = self.scenes.borrow_mut();
        if scenes.remove(name).is_none() {
            bail!("Scene '{}' not found", name);
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

    pub fn ui(&mut self, ctx: &egui::Context) -> Result<()> {
        let current_scene = self.current.clone();
        let mut result: Result<()> = Ok(());
        self.with_current_scene_mut(|scene| {
            match scene
                .ui(ctx)
                .context(format!("UI for scene {current_scene}"))
            {
                Ok(_) => {}
                Err(e) => {
                    warn!(error = ?e, "Error in scene UI");
                    result = Err(e);
                }
            };
        })?;
        result
    }

    pub fn destroy(&mut self) -> Result<()> {
        let mut scenes = self.scenes.borrow_mut();
        for scene in scenes.values_mut() {
            trace!(name = ?scene.name(), "Scene destroyed");
            scene.destroy();
        }
        debug!("SceneManager destroyed");
        Ok(())
    }

    pub fn transfer_to(&mut self, next_scene: String) -> Result<()> {
        if next_scene == SCENE_QUIT {
            self.quit = true;
            debug!("Quit scene recived");
            return Ok(());
        }

        {
            debug!(scene = ?next_scene, "Transferring to scene");
            self.current = next_scene.clone();
            let mut new_current = self.current.clone();
            {
                let scenes = self.scenes.borrow();
                if !scenes.contains_key(&next_scene) {
                    warn!(scene = ?next_scene, "Scene not found, transferring to 'no_scene'");
                    let no_scene = scenes.get("no_scene").ok_or_else(|| {
                        anyhow::anyhow!(
                            "No scene was found, and `no_scene` is not initialized; crashing"
                        )
                        .context("Transferring to 'no_scene'")
                        .context(format!("Transferring to {next_scene}"))
                    })?;
                    new_current = no_scene.name().to_string();
                }
            }
            self.current = new_current;
        }

        self.with_current_scene_mut(|scene| -> Result<()> {
            scene
                .reload()
                .context(format!("Reloading scene {next_scene}"))?;
            trace!(name = ?scene.name(), "Scene reloaded");
            Ok(())
        })??;

        Ok(())
    }

    fn with_current_scene<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Box<dyn Scene>) -> R,
    {
        let scenes = self.scenes.borrow();
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
        let mut scenes = self.scenes.borrow_mut();
        let scene = scenes.get_mut(&current_scene).ok_or_else(|| {
            anyhow::anyhow!("Scene '{}' not found", current_scene)
                .context("Accessing current scene mutably")
        })?;
        Ok(f(scene))
    }
}
