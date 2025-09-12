use deferred::camera::Camera;

use crate::settings::Settings;

#[derive(Clone)]
pub struct GameData {
    pub settings: Settings,
    pub camera: Camera,
}

impl GameData {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            camera: Camera::new([0.0, 0.0], [800.0, 600.0], 50.0),
        }
    }
}
