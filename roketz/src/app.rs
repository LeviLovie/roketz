use deferred::{object::Object, Renderer};
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

use crate::data::GameData;
use utils::prelude::*;

const TARGET_FPS: f32 = 60.0;

pub struct App {
    renderer: Option<Renderer>,
    data: MArc<GameData>,
    last_frame: Instant,
    frame_time: Duration,
}

impl App {
    pub fn new(data: MArc<GameData>) -> Self {
        Self {
            renderer: None,
            data,
            last_frame: Instant::now(),
            frame_time: Duration::from_secs_f32(1.0 / TARGET_FPS),
        }
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
        let window = Window::default_attributes().with_title("Deferred rendering");

        let window = Arc::new(event_loop.create_window(window).unwrap());

        let renderer = pollster::block_on(Renderer::new(window.clone()));
        self.renderer = Some(renderer);

        window.request_redraw();
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let now = Instant::now();
        if now - self.last_frame >= self.frame_time {
            self.last_frame = now;
            if let Some(renderer) = &self.renderer {
                renderer.window.request_redraw();
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
            && renderer.window.id() != window_id
        {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(new_size.width, new_size.height);
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
                let objects = vec![
                    (
                        -25.0,
                        vec![Object {
                            pos: [350.0, 100.0],
                            size: [100.0, 50.0],
                            color: [1.0, 0.0, 0.0, 1.0],
                        }],
                    ),
                    (
                        -10.0,
                        vec![Object {
                            pos: [350.0, 150.0],
                            size: [100.0, 50.0],
                            color: [0.0, 1.0, 0.0, 1.0],
                        }],
                    ),
                    (
                        0.0,
                        vec![Object {
                            pos: [350.0, 200.0],
                            size: [100.0, 50.0],
                            color: [1.0, 1.0, 1.0, 1.0],
                        }],
                    ),
                    (
                        10.0,
                        vec![Object {
                            pos: [350.0, 250.0],
                            size: [100.0, 50.0],
                            color: [0.0, 0.0, 1.0, 1.0],
                        }],
                    ),
                ];

                let camera = {
                    let data = self.data.lock_panic();
                    data.camera
                };

                if let Some(renderer) = &mut self.renderer {
                    renderer.render(objects, &camera);
                }
            }
            _ => {}
        }
    }
}
