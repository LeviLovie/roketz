use macroquad::prelude::*;

pub struct Camera {
    camera: Camera2D,
    pub zoom: f32,
    pub target: Vec2,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            camera: Camera2D::default(),
            zoom: 0.01,
            target: Vec2::ZERO,
        }
    }

    pub fn teleport(&mut self, target: Vec2, zoom: f32) {
        self.target = target;
        self.camera.target = target;
        self.camera.zoom = Vec2::splat(zoom);
    }

    pub fn reset(&mut self) {
        self.target = Vec2::ZERO;
        self.zoom = 0.001;
    }

    pub fn update(&mut self) {
        self.camera.zoom = Self::zoom_vec(self.zoom);
        self.camera.target = self.target;

        if is_key_down(KeyCode::Q) {
            self.zoom *= 1.01;
        } else if is_key_down(KeyCode::E) {
            self.zoom *= 0.99;
        }
        if is_key_down(KeyCode::Up) {
            self.target.y -= 0.01 / self.zoom;
        } else if is_key_down(KeyCode::Down) {
            self.target.y += 0.01 / self.zoom;
        }
        if is_key_down(KeyCode::Left) {
            self.target.x -= 0.01 / self.zoom;
        } else if is_key_down(KeyCode::Right) {
            self.target.x += 0.01 / self.zoom;
        }

        set_camera(&self.camera);
    }

    pub fn zoom_vec(zoom: f32) -> Vec2 {
        Vec2::new(zoom, zoom * screen_width() / screen_height())
    }
}
