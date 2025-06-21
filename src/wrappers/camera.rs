use macroquad::prelude::*;

pub enum CameraType {
    Global,
    Left,
    Right,
    Top,
    Bottom,
}

pub struct Camera {
    camera: Camera2D,
    ty: CameraType,
    pub zoom: f32,
    pub target: Vec2,
}

impl Camera {
    pub fn new(ty: CameraType) -> Self {
        Self {
            camera: Camera2D::default(),
            ty,
            zoom: 0.01,
            target: Vec2::ZERO,
        }
    }

    pub fn change_type(&mut self, ty: CameraType) {
        self.ty = ty;
        match self.ty {
            CameraType::Global => {
                self.camera = Camera2D::default();
            }
            CameraType::Left => {
                self.camera = Camera2D {
                    target: vec2(-screen_width() / 4.0, 0.0),
                    zoom: Vec2::new(0.01, 0.01 * screen_width() / screen_height()),
                    ..Default::default()
                };
            }
            CameraType::Right => {
                self.camera = Camera2D {
                    target: vec2(screen_width() / 4.0, 0.0),
                    zoom: Vec2::new(0.01, 0.01 * screen_width() / screen_height()),
                    ..Default::default()
                };
            }
            CameraType::Top => {
                self.camera = Camera2D {
                    target: vec2(0.0, -screen_height() / 4.0),
                    zoom: Vec2::new(0.01, 0.01 * screen_width() / screen_height()),
                    ..Default::default()
                };
            }
            CameraType::Bottom => {
                self.camera = Camera2D {
                    target: vec2(0.0, screen_height() / 4.0),
                    zoom: Vec2::new(0.01, 0.01 * screen_width() / screen_height()),
                    ..Default::default()
                };
            }
        }
    }

    pub fn teleport(&mut self, target: Vec2, zoom: f32) {
        self.target = target;
        self.camera.target = target;
        self.camera.zoom = Vec2::splat(zoom);
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
