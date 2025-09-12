mod battle;

pub use battle::BattleScene;

use deferred::object::Object;
use utils::prelude::*;

use crate::data::GameData;

pub type Objects = Vec<(f32, Vec<Object>)>;

pub trait Scene: 'static {
    fn name(&self) -> String;

    fn scene(self) -> Box<dyn Scene>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    fn create(data: MArc<GameData>) -> Result<Self>
    where
        Self: Sized;

    fn reload(&mut self) -> Result<()> {
        Ok(())
    }

    fn transfer(&self) -> Option<MArc<Box<dyn Scene>>> {
        None
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Objects {
        vec![]
    }

    fn destroy(&mut self) {}
}

pub struct SceneManager {
    scene: MArc<Box<dyn Scene>>,
}

impl SceneManager {
    #[instrument(skip_all)]
    pub fn new(data: MArc<GameData>) -> Result<Self> {
        let initial_scene = NoScene::create(data.clone())
            .context("Creating NoScene (init scene in the SceneManager")?;
        debug!("Initial scene: {}", initial_scene.name());

        info!("SceneManager created");
        Ok(Self {
            scene: MArc::new(initial_scene.scene(), "Initial Scene"),
        })
    }

    pub fn transfer(&mut self, new_scene: MArc<Box<dyn Scene>>) -> Result<()> {
        let current_name = self.scene.lock()?.name();
        info!(
            "Transferring from scene '{}' to scene '{}'",
            current_name,
            new_scene.lock()?.name()
        );
        self.scene.lock()?.destroy();
        self.scene = new_scene;
        self.scene.lock()?.reload()?;
        Ok(())
    }

    // TODO: Make better use of scene lock (less locks)
    pub fn update(&mut self) -> Result<()> {
        let transfer = {
            let mut scene = self.scene.lock()?;
            scene.update();
            scene.transfer()
        };

        if let Some(new_scene) = transfer {
            self.transfer(new_scene)
                .context("Transferring to new scene")?;
        }

        Ok(())
    }

    pub fn render(&mut self) -> Objects {
        self.scene
            .lock_do(
                |s| s.render(),
                |e| error!("Failed to lock scene for rendering: {}", e),
            )
            .unwrap_or_default()
    }
}

pub struct NoScene {}

impl Scene for NoScene {
    fn name(&self) -> String {
        "NoScene".to_string()
    }

    fn create(_data: MArc<GameData>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
