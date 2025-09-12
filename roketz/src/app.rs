use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::Key,
    platform::modifier_supplement::KeyEventExtModifierSupplement,
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
    last_frame: Instant,
    frame_time: Duration,
    accumulator: Duration,
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

        let fps = data.lock()?.settings.window.fps;

        Ok(Self {
            renderer: None,
            data,
            scenes,
            last_frame: Instant::now(),
            frame_time: Duration::from_secs_f32(1.0 / fps as f32),
            accumulator: Duration::ZERO,
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
        let now = Instant::now();
        let dt = now - self.last_frame;
        self.last_frame = now;
        self.accumulator += dt;

        while self.accumulator >= self.frame_time {
            self.accumulator -= self.frame_time;
            self.scenes.update().unwrap_or_else(|e| {
                error!("Error updating scenes: {}", e);
            });
            if let Some(renderer) = &self.renderer {
                renderer.lock_panic().window.request_redraw();
            }
        }
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
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    let mut data = self.data.lock_panic();
                    match event.key_without_modifiers().as_ref() {
                        Key::Character("w") => {
                            data.camera.pos[1] += 20.0;
                        }
                        Key::Character("s") => {
                            data.camera.pos[1] -= 20.0;
                        }
                        Key::Character("a") => {
                            data.camera.pos[0] -= 20.0;
                        }
                        Key::Character("d") => {
                            data.camera.pos[0] += 20.0;
                        }
                        Key::Character("q") => {
                            data.camera.size[0] *= 1.05;
                            data.camera.size[1] *= 1.05;
                        }
                        Key::Character("e") => {
                            data.camera.size[0] *= 0.95;
                            data.camera.size[1] *= 0.95;
                        }
                        _ => (),
                    }
                }
            }
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
