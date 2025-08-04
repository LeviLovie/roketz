use helpers::error::HandleError;
use macroquad::prelude::*;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::ecs::cs::Transform;

#[derive(Clone, Eq, PartialEq)]
pub enum CameraType {
    Global,
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone)]
pub struct Camera {
    camera: Arc<Mutex<Camera2D>>,
    pub ty: CameraType,
    pub zoom: f32,
}

impl Camera {
    pub fn new(ty: CameraType) -> Self {
        Self {
            camera: Arc::new(Mutex::new(Camera2D::default())),
            ty,
            zoom: 0.01,
        }
    }

    pub fn borrow_camera(&self) -> MutexGuard<'_, Camera2D> {
        self.camera.lock().handle("Failed to lock camera mutex")
    }

    pub fn borrow_camera_mut(&mut self) -> MutexGuard<'_, Camera2D> {
        self.camera.lock().handle("Failed to lock camera mutex")
    }

    pub fn change_type(&mut self, ty: CameraType) {
        self.ty = ty;
    }

    pub fn set_target(&mut self, target: Vec2) {
        match self.ty {
            CameraType::Global => {
                self.borrow_camera_mut().target = target;
            }
            CameraType::Left | CameraType::Right | CameraType::Top | CameraType::Bottom => {
                self.borrow_camera_mut().target = target;
            }
        }
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        let (w, h) = match self.borrow_camera().viewport {
            Some((_, _, width, height)) => (width as f32, height as f32),
            None => (screen_width(), screen_height()),
        };

        let aspect_ratio = w / h;
        self.borrow_camera_mut().zoom = vec2(zoom, zoom * aspect_ratio);
    }

    pub fn update(&mut self, target: &Transform) {
        match self.ty {
            CameraType::Global => {
                self.borrow_camera_mut().viewport = None;
            }
            CameraType::Left => {
                self.borrow_camera_mut().viewport =
                    Some((0, 0, screen_width() as i32 / 2, screen_height() as i32));
            }
            CameraType::Right => {
                self.borrow_camera_mut().viewport = Some((
                    screen_width() as i32 / 2,
                    0,
                    screen_width() as i32 / 2,
                    screen_height() as i32,
                ));
            }
            CameraType::Top => {
                self.borrow_camera_mut().viewport =
                    Some((0, 0, screen_width() as i32, screen_height() as i32 / 2));
            }
            CameraType::Bottom => {
                self.borrow_camera_mut().viewport = Some((
                    0,
                    screen_height() as i32 / 2,
                    screen_width() as i32,
                    screen_height() as i32 / 2,
                ));
            }
        }

        self.set_target(target.pos);
        self.set_zoom(self.zoom);

        if is_key_down(KeyCode::T) {
            self.zoom *= 1.01;
        } else if is_key_down(KeyCode::Y) {
            self.zoom *= 0.99;
        }
    }

    pub fn set(&self) {
        set_camera(&*self.borrow_camera());
    }

    pub fn zoom_vec(zoom: f32) -> Vec2 {
        Vec2::new(zoom, zoom * screen_width() / screen_height())
    }
}
