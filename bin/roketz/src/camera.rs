use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use ecs::cs::Transform;

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
    pub id: Entity,
}

impl Camera {
    pub fn new(id: Entity) -> Self {
        Self {
            camera: Camera2D::default(),
            ty: CameraType::Global,
            zoom: 0.01,
            id,
        }
    }

    pub fn change_type(&mut self, ty: CameraType) {
        self.ty = ty;
    }

    pub fn set_target(&mut self, target: Vec2) {
        match self.ty {
            CameraType::Global => {
                self.camera.target = target;
            }
            CameraType::Left | CameraType::Right | CameraType::Top | CameraType::Bottom => {
                self.camera.target = target;
            }
        }
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        let (w, h) = match self.camera.viewport {
            Some((_, _, width, height)) => (width as f32, height as f32),
            None => (screen_width(), screen_height()),
        };

        let aspect_ratio = w / h;
        self.camera.zoom = vec2(zoom, zoom * aspect_ratio);
    }

    pub fn update(&mut self, world: &mut World) {
        match self.ty {
            CameraType::Global => {
                self.camera.viewport = None;
            }
            CameraType::Left => {
                self.camera.viewport =
                    Some((0, 0, screen_width() as i32 / 2, screen_height() as i32));
            }
            CameraType::Right => {
                self.camera.viewport = Some((
                    screen_width() as i32 / 2,
                    0,
                    screen_width() as i32 / 2,
                    screen_height() as i32,
                ));
            }
            CameraType::Top => {
                self.camera.viewport =
                    Some((0, 0, screen_width() as i32, screen_height() as i32 / 2));
            }
            CameraType::Bottom => {
                self.camera.viewport = Some((
                    0,
                    screen_height() as i32 / 2,
                    screen_width() as i32,
                    screen_height() as i32 / 2,
                ));
            }
        }

        let target = world
            .get::<Transform>(self.id)
            .unwrap_or(&Transform::default())
            .pos;
        self.set_target(target);
        self.set_zoom(self.zoom);

        if is_key_down(KeyCode::T) {
            self.zoom *= 1.01;
        } else if is_key_down(KeyCode::Y) {
            self.zoom *= 0.99;
        }
    }

    pub fn set(&self) {
        set_camera(&self.camera);
    }

    pub fn zoom_vec(zoom: f32) -> Vec2 {
        Vec2::new(zoom, zoom * screen_width() / screen_height())
    }
}
