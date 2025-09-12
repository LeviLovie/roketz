use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Window {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub title: String,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            width: 1600,
            height: 1200,
            fullscreen: false,
            title: "Roketz".to_string(),
        }
    }
}
