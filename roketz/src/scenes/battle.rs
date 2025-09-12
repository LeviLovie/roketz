use deferred::object::Object;
use utils::prelude::*;

use super::{Objects, Scene};
use crate::data::GameData;

pub struct BattleScene {
    data: MArc<GameData>,
}

impl Scene for BattleScene {
    fn name(&self) -> String {
        "BattleScene".to_string()
    }

    #[instrument(skip_all)]
    fn create(data: MArc<GameData>) -> Result<Self>
    where
        Self: Sized,
    {
        info!("Creating BattleScene");
        Ok(Self { data })
    }

    fn update(&mut self) {
        if self.data.lock_panic().exit {
            info!("Exit flag set, exiting");
            return;
        }
    }

    fn render(&mut self) -> Objects {
        vec![
            (
                -25.0,
                vec![Object {
                    pos: [350.0, 100.0],
                    size: [100.0, 50.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                }],
            ),
            (
                -10.0,
                vec![Object {
                    pos: [350.0, 150.0],
                    size: [100.0, 50.0],
                    color: [0.0, 1.0, 0.0, 1.0],
                }],
            ),
            (
                0.0,
                vec![Object {
                    pos: [350.0, 200.0],
                    size: [100.0, 50.0],
                    color: [1.0, 1.0, 1.0, 1.0],
                }],
            ),
            (
                10.0,
                vec![Object {
                    pos: [350.0, 250.0],
                    size: [100.0, 50.0],
                    color: [0.0, 0.0, 1.0, 1.0],
                }],
            ),
        ]
    }
}
