use anyhow::{bail, Context, Result};
use helpers::error::HandleError;
use rdss::Loader;
use std::sync::{Arc, Mutex};
use tracing::debug;

mod fields;
mod simple;

pub mod kinds {
    pub use super::simple::Simple;
}

const SPRITES_DIR: &str = "sprites";
const SPRITES_FILE: &str = "sprites/sprites.kdl";

#[derive(Debug, Eq, PartialEq, Clone, knus::Decode)]
pub enum SpriteKind {
    Simple(simple::Simple),
}

pub struct Sprites {
    sprites: Vec<SpriteKind>,
}

impl Sprites {
    pub fn load(assets: Arc<Mutex<Loader>>) -> Result<Self> {
        let sprites_file = assets
            .lock()
            .handle("Failed to lock assets mutex")
            .read(SPRITES_FILE)
            .context("Faile to read sprites file")?;

        let sprites: Vec<SpriteKind> = knus::parse(SPRITES_FILE, &sprites_file).map_err(|e| {
            anyhow::anyhow!(
                "Failed to parse sprites config:\n{:?}",
                miette::Report::new(e)
            )
        })?;

        debug!("Loaded {} sprites successfully", sprites.len());

        Ok(Sprites { sprites })
    }

    pub fn find(&self, name: &str) -> Result<SpriteKind> {
        for (i, sprite) in self.sprites.iter().enumerate() {
            match sprite {
                SpriteKind::Simple(simple) => {
                    if simple.name == name {
                        return Ok(self.sprites[i].clone());
                    }
                }
            }
        }

        bail!("No sprite with this name exists: {}", name)
    }

    pub fn to_absolute_path(path: String) -> String {
        format!("{SPRITES_DIR}/{path}")
    }
}
