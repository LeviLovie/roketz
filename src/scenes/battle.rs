use crate::game::{GameData, Scene};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

#[allow(unused)]
pub struct Battle {
    data: Arc<Mutex<GameData>>,
}

impl Scene for Battle {
    fn create(data: Arc<Mutex<GameData>>) -> Self {
        Self { data }
    }

    fn name(&self) -> &str {
        "Battle"
    }

    fn render(&self) {
        let text = "Battle";
        draw_text(
            text,
            screen_width() / 2.0 - measure_text(text, None, 64, 1.0).width / 2.0,
            screen_height() / 2.0,
            64.0,
            WHITE,
        );
    }
}
