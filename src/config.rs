use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsConfig {
    pub width: u32,
    pub height: u32,
    pub scale: u32,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        GraphicsConfig {
            width: 800,
            height: 600,
            scale: 8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub graphics: GraphicsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            graphics: GraphicsConfig::default(),
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

    pub fn check_if_exists_and_create(&self) -> Result<()> {
        if !self.exists() {
            let config_dir = dirs::config_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("my_app");
            std::fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
            let config_path = self.get_config_file_path();
            std::fs::write(
                config_path,
                toml::to_string(&self).context("Failed to serialize config")?,
            )
            .context("Failed to write config file")?;
        }

        Ok(())
    }

    pub fn load(&self) -> Result<Self> {
        let config_path = self.get_config_file_path();

        if !std::path::Path::new(&config_path).exists() {
            return Err(anyhow::anyhow!("Config file does not exist"));
        }
        let config_content =
            std::fs::read_to_string(config_path).context("Failed to read config file")?;
        let config: Config =
            toml::from_str(&config_content).context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = self.get_config_file_path();

        std::fs::write(
            config_path,
            toml::to_string(&self).context("Failed to serialize config")?,
        )
        .context("Failed to write config file")?;

        Ok(())
    }

    fn get_config_file_path(&self) -> String {
        let mut path = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        path.push("my_app");
        path.push("config.toml");
        path.to_str().unwrap().to_string()
    }
}
