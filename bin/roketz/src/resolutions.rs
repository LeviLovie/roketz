use anyhow::{Context, Result};
use helpers::error::HandleError;
use rdss::Loader;
use std::sync::{Arc, Mutex};
use tracing::debug;

const RESOLUTIONS_FILE: &str = "resolutions.kdl";

#[derive(Clone, Eq, PartialEq, Debug, knus::Decode)]
pub struct Resolution {
    #[knus(property)]
    pub w: u32,
    #[knus(property)]
    pub h: u32,
    #[knus(property)]
    pub a: String,
    #[knus(property)]
    pub t: String,
}

#[derive(Clone)]
pub struct Resolutions {
    pub resolutions: Vec<Resolution>,
}

impl Resolutions {
    pub fn load(assets: Arc<Mutex<Loader>>) -> Result<Self> {
        let resolutions_file = assets
            .lock()
            .handle("Failed to lock assets mutex")
            .read(RESOLUTIONS_FILE)
            .context("Faile to read sprites file")?;

        let resolutions: Vec<Resolution> = knus::parse(RESOLUTIONS_FILE, &resolutions_file)
            .map_err(|e| {
                anyhow::anyhow!(
                    "Failed to parse resolutions config:\n{:?}",
                    miette::Report::new(e)
                )
            })?;

        debug!("Loaded {} resulotions successfully", resolutions.len());

        Ok(Resolutions { resolutions })
    }

    pub fn find_by_size(&self, w: u32, h: u32) -> Option<Resolution> {
        self.resolutions
            .iter()
            .find(|resolution| resolution.w == w && resolution.h == h)
            .cloned()
    }
}
