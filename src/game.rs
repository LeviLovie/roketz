use anyhow::{Context, Result};
use pixels::Pixels;
use rasset::prelude::Registry;
use std::sync::Arc;
use tracing::{debug, trace};
use winit::{event::WindowEvent, window::Window};

use crate::Config;

pub struct Game<'a> {
    pub window: Arc<Window>,
    pub size: (u32, u32),
    pixels: Pixels<'a>,

    _assets: Arc<Registry>,
    config: Config,
}

impl<'a> Game<'a> {
    #[tracing::instrument(skip_all)]
    pub fn new(config: Config, window: Arc<Window>) -> Result<Self> {
        let assets = {
            let exec_dir =
                std::env::current_exe().context("Failed to get current executable directory")?;
            let assets_path = exec_dir
                .parent()
                .context("Failed to get parent directory of executable")?
                .join(&config.assets);
            if !assets_path.exists() {
                return Err(anyhow::anyhow!(
                    "Assets file does not exist at {}",
                    assets_path.display()
                ));
            }

            let assets_binary = std::fs::read(&assets_path).context(format!(
                "Failed to read assets from {}",
                assets_path.display()
            ))?;

            let registry = assets::registry(assets_binary)?;

            Arc::new(registry)
        };

        let window_size = window.inner_size();
        let size = (window_size.width, window_size.height);
        let surface_texture = pixels::SurfaceTexture::new(size.0, size.1, window.clone());
        let pixels = Pixels::new(size.0, size.1, surface_texture)
            .context("Failed to create Pixels instance")?;

        debug!("Game created");
        Ok(Game {
            _assets: assets,
            config,
            window,
            pixels,
            size,
        })
    }

    #[tracing::instrument(skip_all)]
    pub fn resize(&mut self, new_size: (u32, u32)) -> Result<()> {
        self.size = Self::scale_size(new_size.0, new_size.1, self.config.graphics.scale as f32);
        trace!(size = ?self.size, "Resizing buffer");
        self.pixels
            .resize_buffer(self.size.0, self.size.1)
            .context("Failed to resize buffer")?;
        trace!(size = ?new_size, "Resizing surface");
        self.pixels
            .resize_surface(new_size.0, new_size.1)
            .context("Failed to resize surface")?;
        Ok(())
    }

    fn scale_size(width: u32, height: u32, scale: f32) -> (u32, u32) {
        let scaled_width = (width as f32 / scale).round() as u32;
        let scaled_height = (height as f32 / scale).round() as u32;
        (scaled_width, scaled_height)
    }

    #[tracing::instrument(skip_all)]
    pub fn handle_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: winit::event::WindowEvent,
    ) -> Result<()> {
        match event {
            WindowEvent::RedrawRequested => {
                self.draw()?;
                self.pixels.render()?;
                self.window.request_redraw();
            }
            WindowEvent::Resized(new_size) => {
                trace!("Handling window resize event");
                self.resize((new_size.width, new_size.height))?;
            }
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                trace!("Handling close_requested/destroy event");
                event_loop.exit();
            }
            _ => {}
        }

        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        let frame = self.pixels.frame_mut();

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let pixel_index = (y * self.size.0 + x) as usize * 4;
                if pixel_index + 3 >= frame.len() {
                    tracing::warn!(
                        "Pixel index out of bounds: {}, size: {:?}",
                        pixel_index,
                        self.size
                    );
                    continue;
                }

                let distance = ((x as f32 - self.size.0 as f32 / 2.0).powi(2)
                    + (y as f32 - self.size.1 as f32 / 2.0).powi(2))
                .sqrt();

                if distance < 25.0 {
                    frame[pixel_index] = 255;
                    frame[pixel_index + 1] = 0;
                    frame[pixel_index + 2] = 0;
                    frame[pixel_index + 3] = 255;
                } else {
                    frame[pixel_index] = 0;
                    frame[pixel_index + 1] = 0;
                    frame[pixel_index + 2] = 0;
                    frame[pixel_index + 3] = 255;
                }
            }
        }

        Ok(())
    }
}
