use deferred::Camera;

use crate::{inputs::Inputs, settings::Settings};
use utils::prelude::*;

#[derive(Clone)]
pub struct GameData {
    pub settings: Settings,
    pub exit: bool,
    pub camera: Camera,
    pub inputs: MArc<Inputs>,
}

impl GameData {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            exit: false,
            camera: Camera::new([0.0, 0.0], [800.0, 600.0], 50.0),
            inputs: MArc::new(Inputs::default(), "GameData.inputs"),
        }
    }
}
