use macroquad::prelude::*;

pub struct Camera {
    camera: Camera2D,
    target: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            camera: Camera2D {
                zoom: Self::zoom_vec(0.003),
                target: Vec2::new(0.0, 0.0),
                ..Default::default()
            },
            target: Vec2::ZERO,
        }
    }

    pub fn teleport(&mut self, target: Vec2, zoom: f32) {
        self.target = target;
        self.camera.target = target;
        self.camera.zoom = Vec2::splat(zoom);
    }

    pub fn set_target(&mut self, target: Vec2) {
        self.target = target;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.camera.zoom = Vec2::splat(zoom);
    }

    pub fn reset(&mut self) {
        self.target = Vec2::ZERO;
        self.camera = Camera2D::default();
    }

    pub fn update(&mut self) {
        self.camera.target = self.target;
        set_camera(&self.camera);
    }

    fn zoom_vec(zoom: f32) -> Vec2 {
        Vec2::new(zoom, zoom * screen_width() / screen_height())
    }
}
