use macroquad::prelude::*;

pub struct Camera {
    camera: Camera2D,
    pub zoom: f32,
    pub target: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            camera: Camera2D::default(),
            zoom: 0.001,
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
            self.zoom += 0.00005;
        } else if is_key_down(KeyCode::E) {
            self.zoom -= 0.00005;
        }
        if is_key_down(KeyCode::Up) {
            self.target.y -= 10.0;
        } else if is_key_down(KeyCode::Down) {
            self.target.y += 10.0;
        }
        if is_key_down(KeyCode::Left) {
            self.target.x -= 10.0;
        } else if is_key_down(KeyCode::Right) {
            self.target.x += 10.0;
        }

        set_camera(&self.camera);
    }

    fn zoom_vec(zoom: f32) -> Vec2 {
        Vec2::new(zoom, zoom * screen_width() / screen_height())
    }
}
