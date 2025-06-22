use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::{debug, trace};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Window {
    pub width: u32,
    pub height: u32,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Graphics {
    pub scale: u32,
}

impl Default for Graphics {
    fn default() -> Self {
        Self { scale: 4 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CollisionResolution {
    pub nearby_nodes_radius: f32,
    pub nearby_nodes_radius_bullet: f32,
}

impl Default for CollisionResolution {
    fn default() -> Self {
        Self {
            nearby_nodes_radius: 30.0,
            nearby_nodes_radius_bullet: 5.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Physics {
    pub bvh_depth: u32,
    pub max_crash_velocity: f32,
    pub collisions: CollisionResolution,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            bvh_depth: 8,
            max_crash_velocity: 50.0,
            collisions: CollisionResolution::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub window: Window,
    pub graphics: Graphics,
    pub physics: Physics,
    pub assets: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: Window::default(),
            graphics: Graphics::default(),
            physics: Physics::default(),
            assets: "assets.bin".to_string(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
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
            let pretty = ron::ser::PrettyConfig::new()
                .depth_limit(10)
                .separate_tuple_members(true)
                .enumerate_arrays(true);
            let ron_string = ron::ser::to_string_pretty(&Config::default(), pretty).unwrap();
            std::fs::write(config_path, ron_string).context("Failed to write config file")?;
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
            ron::from_str(&config_content).context("Failed to parse config file")?;

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
            ron::ser::to_string(self).context("Failed to serialize config")?,
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
        path.push("config.ron");
        path.to_str().unwrap().to_string()
    }

    fn app_name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }
}
