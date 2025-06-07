use anyhow::{Context, Result};
use pixels::{Pixels, SurfaceTexture};
use roketz::Config;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

pub struct AppData<'a> {
    window: Arc<Window>,
    window_size: winit::dpi::PhysicalSize<u32>,
    size: (u32, u32),
    pixels: Pixels<'a>,
}

pub struct App<'a> {
    data: Option<AppData<'a>>,
    config: Config,
}

impl<'a> App<'a> {
    fn init(&mut self, _event_loop: &ActiveEventLoop) {
        let data: &mut AppData = self.data.as_mut().expect("App data not initialized");
        let frame = data.pixels.frame_mut();

        for y in 0..data.size.1 {
            for x in 0..data.size.0 {
                let pixel_index = (y * data.size.0 + x) as usize * 4;
                let distance = ((x as f32 - data.size.0 as f32 / 2.0).powi(2)
                    + (y as f32 - data.size.1 as f32 / 2.0).powi(2))
                .sqrt();
                if distance < 25.0 {
                    frame[pixel_index] = 255; // Red
                    frame[pixel_index + 1] = 0; // Green
                    frame[pixel_index + 2] = 0; // Blue
                    frame[pixel_index + 3] = 255; // Alpha
                } else {
                    frame[pixel_index] = 0; // Red
                    frame[pixel_index + 1] = 0; // Green
                    frame[pixel_index + 2] = 0; // Blue
                    frame[pixel_index + 3] = 255; // Alpha
                }
            }
        }
    }

    fn draw_frame(&mut self) {
        let data: &mut AppData = self.data.as_mut().expect("App data not initialized");
        let frame = data.pixels.frame_mut();

        for y in 0..data.size.1 {
            for x in 0..data.size.0 {
                let pixel_index = (y * data.size.0 + x) as usize * 4;
                let distance = ((x as f32 - data.size.0 as f32 / 2.0).powi(2)
                    + (y as f32 - data.size.1 as f32 / 2.0).powi(2))
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
    }

    fn calc_scaled_size(width: u32, height: u32, scale_factor: u32) -> (u32, u32) {
        (width / scale_factor, height / scale_factor)
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Winit Window")
            .with_inner_size(LogicalSize::new(
                self.config.graphics.width,
                self.config.graphics.height,
            ));

        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .context("Failed to create window")
                .unwrap(),
        );

        let window_size = window.inner_size();
        let size = App::calc_scaled_size(
            window_size.width,
            window_size.height,
            self.config.graphics.scale,
        );
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.clone());
        let pixels = Pixels::new(size.0, size.1, surface_texture).expect("Failed to create pixels");

        self.data = Some(AppData {
            window,
            window_size,
            size,
            pixels,
        });

        self.init(event_loop);

        self.data
            .as_mut()
            .expect("App data not initialized")
            .window
            .request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                self.draw_frame();

                let data: &mut AppData = self.data.as_mut().expect("App data not initialized");
                data.pixels.render().expect("Failed to render pixels");
                data.window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                let scale = self.config.graphics.scale;
                let data: &mut AppData = self.data.as_mut().expect("App data not initialized");

                data.window_size = size;
                data.size = App::calc_scaled_size(size.width, size.height, scale);
                data.pixels
                    .resize_surface(size.width, size.height)
                    .expect("Failed to resize surface");
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}

pub fn create_and_run() -> Result<()> {
    let config = Config::new();
    config
        .check_if_exists_and_create()
        .context("Failed to check or create configuration")?;
    config.load().context("Failed to load configuration")?;

    let mut app = App { data: None, config };
    let event_loop = EventLoop::new().context("Failed to create event loop")?;
    event_loop
        .run_app(&mut app)
        .context("Failed to run application")?;

    Ok(())
}
