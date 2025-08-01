use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::{debug, trace};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub window: Window,
    pub assets: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window: Window::default(),
            assets: "assets.bin".to_string(),
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exists(&self) -> Result<bool> {
        let config_path = self.get_config_file_path()?;
        Ok(std::path::Path::new(&config_path).exists())
    }

    #[tracing::instrument(skip_all)]
    pub fn check_if_exists_and_create(&self) -> Result<()> {
        if !self.exists()? {
            let config_path = self.get_config_file_path()?;
            debug!(
                path = ?config_path,
                "Settings file does not exist, creating a new one",
            );
            let config_dir = dirs::config_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(self.app_name());
            std::fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
            let pretty = ron::ser::PrettyConfig::new()
                .depth_limit(10)
                .separate_tuple_members(true)
                .enumerate_arrays(true);
            let ron_string = ron::ser::to_string_pretty(&Settings::default(), pretty)
                .context("Failed to serialize config")?;
            std::fs::write(config_path, ron_string).context("Failed to write config file")?;
            trace!("Settings file created");
        }

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub fn load(&self) -> Result<Self> {
        let config_path = self.get_config_file_path()?;
        let start = std::time::Instant::now();

        if !std::path::Path::new(&config_path).exists() {
            return Err(anyhow::anyhow!("Settings file does not exist"));
        }
        let config_content =
            std::fs::read_to_string(&config_path).context("Failed to read config file")?;
        let config: Settings =
            ron::from_str(&config_content).context("Failed to parse config file")?;

        debug!(
            path = ?config_path,
            "Settings loaded successfully in {:.2}ms",
            start.elapsed().as_micros() as f32 / 1000.0
        );
        Ok(config)
    }

    #[tracing::instrument(skip_all)]
    pub fn save(&self) -> Result<()> {
        let config_path = self.get_config_file_path()?;
        let start = std::time::Instant::now();

        std::fs::write(
            config_path,
            ron::ser::to_string(self).context("Failed to serialize config")?,
        )
        .context("Failed to write config file")?;

        debug!(
            "Settings saved successfully in {:.2}ms",
            start.elapsed().as_micros() as f32 / 1000.0
        );
        Ok(())
    }

    fn get_config_file_path(&self) -> Result<String> {
        let mut path = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        path.push(self.app_name());
        path.push("config.ron");
        Ok(path
            .to_str()
            .context("Failed to convert config path to string")?
            .to_string())
    }

    fn app_name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }
}
