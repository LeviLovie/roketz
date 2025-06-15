use rasset::prelude::*;

asset_def! {
    Sprite: {
        width: u32,
        height: u32,
        texture: String,
    }
}

pub fn registry(binary: Vec<u8>) -> Result<Registry, Error> {
    Registry::builder().reg_type::<Sprite>().load(&binary)
}

#[cfg(feature = "declare")]
pub mod declare {
    use super::*;

    asset_file!("../assets/assets.ron");
}
