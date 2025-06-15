use crate::game::GameData;
use std::sync::{Arc, Mutex};

pub trait Scene: Send + Sync {
    fn create(data: Arc<Mutex<GameData>>) -> Self
    where
        Self: Sized;
}
