use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use crate::{
    data::GameData,
    scenes::{BattleScene, Scene, SceneManager},
};
use deferred::Renderer;
use utils::prelude::*;

pub struct App {
    renderer: Option<MArc<Renderer>>,
    data: MArc<GameData>,
    scenes: SceneManager,
}

impl App {
    pub fn new(data: MArc<GameData>) -> Result<Self> {
        let mut scenes = SceneManager::new(data.clone()).context("Creating SceneManager")?;
        scenes
            .transfer(MArc::new(
                Box::new(BattleScene::create(data.clone()).context("Creating BattleScene")?),
                "Battle Scene",
            ))
            .context("Transferring to BattleScene")?;

        Ok(Self {
            renderer: None,
            data,
            scenes,
        })
    }

    #[instrument(skip_all)]
    pub fn run(&mut self) -> Result<()> {
        let event_loop = EventLoop::new().context("Creating EventLoop")?;
        debug!("EventLoop created");

        info!("Starting EventLoop");
        event_loop.run_app(self).context("Running App")?;

        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let settings = &self.data.lock_panic().settings;
        let window = Window::default_attributes()
            .with_title(settings.window.title.clone())
            .with_inner_size(winit::dpi::PhysicalSize::new(
                settings.window.width,
                settings.window.height,
            ))
            .with_fullscreen(if settings.window.fullscreen {
                Some(winit::window::Fullscreen::Borderless(None))
            } else {
                None
            });

        let window = Arc::new(event_loop.create_window(window).unwrap());

        let renderer = pollster::block_on(Renderer::new(window.clone()));
        self.renderer = Some(MArc::new(renderer, "Renderer"));

        window.request_redraw();
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.scenes.update().unwrap_or_else(|e| {
            error!("Error updating scenes: {}", e);
        });
        if let Some(renderer) = &self.renderer {
            renderer.lock_panic().window.request_redraw();
        }
        self.data.lock_panic().inputs.lock_panic().pre_update();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(renderer) = &self.renderer
            && renderer.lock_panic().window.id() != window_id
        {
            return;
        }

        if self.data.lock_panic().exit {
            event_loop.exit();
            return;
        }

        self.data.lock_panic().inputs.lock_panic().update(&event);

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer
                        .lock_panic()
                        .resize(new_size.width, new_size.height);
                }
            }
            // WindowEvent::KeyboardInput { event, .. } => {}
            WindowEvent::RedrawRequested => {
                let camera = {
                    let data = self.data.lock_panic();
                    data.camera
                };

                if let Some(renderer) = &self.renderer {
                    self.scenes.render(renderer.clone());
                    renderer.lock_panic().render(&camera);
                }
            }
            _ => {}
        }
    }
}
