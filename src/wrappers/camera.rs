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
                    viewport: Some((0, 0, screen_width() as i32 / 2, screen_height() as i32)),
                    ..Default::default()
                };
            }
            CameraType::Right => {
                self.camera = Camera2D {
                    viewport: Some((
                        screen_width() as i32 / 2,
                        0,
                        screen_width() as i32 / 2,
                        screen_height() as i32,
                    )),
                    ..Default::default()
                };
            }
            CameraType::Top => {
                self.camera = Camera2D {
                    viewport: Some((0, 0, screen_width() as i32, screen_height() as i32 / 2)),
                    ..Default::default()
                };
            }
            CameraType::Bottom => {
                self.camera = Camera2D {
                    viewport: Some((
                        0,
                        screen_height() as i32 / 2,
                        screen_width() as i32,
                        screen_height() as i32 / 2,
                    )),
                    ..Default::default()
                };
            }
        }
    }

    pub fn set_target(&mut self, target: Vec2) {
        match self.ty {
            CameraType::Global => {
                self.camera.target = target;
            }
            CameraType::Left => {
                self.camera.target = vec2(-screen_width() / 4.0, target.y);
            }
            CameraType::Right => {
                self.camera.target = vec2(screen_width() / 4.0, target.y);
            }
            CameraType::Top => {
                self.camera.target = vec2(target.x, -screen_height() / 4.0);
            }
            CameraType::Bottom => {
                self.camera.target = vec2(target.x, screen_height() / 4.0);
            }
        }
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        let screen_ration = screen_width() / screen_height();
        self.camera.zoom = vec2(zoom, zoom * screen_ration);
    }

    pub fn teleport(&mut self, target: Vec2, zoom: f32) {
        self.target = target;
        self.camera.target = target;
        self.camera.zoom = Vec2::splat(zoom);
    }

    pub fn update(&mut self) {
        self.set_target(self.target);
        self.set_zoom(self.zoom);

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
    }

    pub fn set(&self) {
        set_camera(&self.camera);
    }

    pub fn zoom_vec(zoom: f32) -> Vec2 {
        Vec2::new(zoom, zoom * screen_width() / screen_height())
    }
}
