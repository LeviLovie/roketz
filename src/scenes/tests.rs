use anyhow::{Context, Result};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::game::{GameData, Scene};

pub struct Tests {
    _data: Arc<Mutex<GameData>>,
}

impl Scene for Tests {
    fn create(data: Option<Arc<Mutex<GameData>>>) -> Result<Self> {
        let data = data.context("Tests scene requires GameData")?.clone();

        Ok(Self { _data: data })
    }

    fn name(&self) -> &str {
        "Tests"
    }

    fn render(&self) {
        clear_background(DARKGRAY);
    }
}
