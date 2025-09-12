mod window;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use utils::prelude::*;
use window::Window;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub window: Window,
}

impl Settings {
    #[instrument(skip_all)]
    pub fn load() -> Result<Self> {
        let path = Self::get_config_path().context("Getting config path")?;

        debug!(?path, "Loading settings");
        if !path.exists() {
            warn!("Settings file does not exist, creating a default one");
            Self::create_default().context("Creating default settings")?;
            debug!("Default settings created");
        }

        let ron = std::fs::read_to_string(&path).context("Reading settings file")?;
        let settings: Self = ron::de::from_str(&ron).context("Deserializing settings from RON")?;

        info!("Settings loaded");
        Ok(settings)
    }

    #[instrument(skip_all)]
    pub fn save(&self) -> Result<()> {
        let path = Self::get_config_path().context("Getting config path")?;
        let ron = ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default())
            .context("Serializing settings to RON")?;
        std::fs::write(&path, ron).context("Writing settings to file")?;

        info!("Settings saved");
        Ok(())
    }

    #[instrument(skip_all)]
    fn create_default() -> Result<()> {
        debug!("Creating default settings");
        let default_settings = Self::default();
        let ron = ron::ser::to_string_pretty(&default_settings, ron::ser::PrettyConfig::default())
            .context("Serializing default settings to RON")?;
        let dir = Self::get_config_dir().context("Getting config dir")?;
        let path = Self::get_config_path().context("Getting config path")?;
        std::fs::create_dir_all(dir).context("Creating config directory")?;
        std::fs::write(&path, ron).context("Writing default settings to file")?;

        info!("Default settings created");
        Ok(())
    }

    #[instrument(skip_all)]
    fn get_config_dir() -> Result<PathBuf> {
        let mut path = dirs::config_dir().context("Getting config directory")?;
        path.push(env!("CARGO_PKG_NAME"));
        Ok(path)
    }

    #[instrument(skip_all)]
    fn get_config_path() -> Result<PathBuf> {
        let mut path = Self::get_config_dir()?;
        path.push("settings.ron");
        Ok(path)
    }
}
