use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Window {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            fullscreen: false,
        }
    }
}
