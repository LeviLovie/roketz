use super::Scene;
use crate::game::GameData;
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

pub struct NoScene;

impl Scene for NoScene {
    fn create(_data: Arc<Mutex<GameData>>) -> Self {
        Self
    }

    fn name(&self) -> &str {
        "no_scene"
    }

    fn render(&self) {
        let text = "No scene";
        draw_text(
            text,
            screen_width() / 2.0 - measure_text(text, None, 64, 1.0).width / 2.0,
            screen_height() / 2.0,
            64.0,
            WHITE,
        );

        let text = "No scene was initialized";
        draw_text(
            text,
            screen_width() / 2.0 - measure_text(text, None, 32, 1.0).width / 2.0,
            screen_height() / 2.0 + 16.0 * 1.0 + 8.0 * 1.0,
            32.0,
            WHITE,
        );

        let text = "Or indexed scene does not exists";
        draw_text(
            text,
            screen_width() / 2.0 - measure_text(text, None, 32, 1.0).width / 2.0,
            screen_height() / 2.0 + 16.0 * 2.0 + 8.0 * 2.0,
            32.0,
            WHITE,
        );
    }
}
