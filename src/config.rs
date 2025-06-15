use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::{debug, trace};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub scale: u32,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        GraphicsConfig {
            title: String::from("Roketz"),
            width: 800,
            height: 600,
            scale: 8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub graphics: GraphicsConfig,
    pub assets: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            graphics: GraphicsConfig::default(),
            assets: String::from("assets.bin"),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn exists(&self) -> bool {
        let config_path = self.get_config_file_path();
        std::path::Path::new(&config_path).exists()
    }

    #[tracing::instrument(skip_all)]
    pub fn check_if_exists_and_create(&self) -> Result<()> {
        if !self.exists() {
            let config_path = self.get_config_file_path();
            debug!(
                path = ?config_path,
                "Config file does not exist, creating a new one",
            );
            let config_dir = dirs::config_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(self.app_name());
            std::fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
            std::fs::write(
                config_path,
                toml::to_string(&self).context("Failed to serialize config")?,
            )
            .context("Failed to write config file")?;
            trace!("Config file created");
        }

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub fn load(&self) -> Result<Self> {
        let config_path = self.get_config_file_path();
        let start = std::time::Instant::now();

        if !std::path::Path::new(&config_path).exists() {
            return Err(anyhow::anyhow!("Config file does not exist"));
        }
        let config_content =
            std::fs::read_to_string(&config_path).context("Failed to read config file")?;
        let config: Config =
            toml::from_str(&config_content).context("Failed to parse config file")?;

        debug!(
            path = ?config_path,
            "Config loaded successfully in {:.2}ms",
            start.elapsed().as_micros() as f32 / 1000.0
        );
        Ok(config)
    }

    #[tracing::instrument(skip_all, fields(config_path = self.get_config_file_path()))]
    pub fn save(&self) -> Result<()> {
        let config_path = self.get_config_file_path();
        let start = std::time::Instant::now();

        std::fs::write(
            config_path,
            toml::to_string(&self).context("Failed to serialize config")?,
        )
        .context("Failed to write config file")?;

        debug!(
            "Config saved successfully in {:.2}ms",
            start.elapsed().as_micros() as f32 / 1000.0
        );
        Ok(())
    }

    fn get_config_file_path(&self) -> String {
        let mut path = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        path.push(self.app_name());
        path.push("config.toml");
        path.to_str().unwrap().to_string()
    }

    fn app_name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }
}
