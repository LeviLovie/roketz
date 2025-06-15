use rasset::prelude::*;

asset_def! {
    Sprite: {
        size: (u32, u32),
        texture: Vec<u8>,
    }
}

pub fn registry(binary: Vec<u8>) -> Result<Registry, Error> {
    Registry::builder().reg_type::<Sprite>().load(&binary)
}

#[cfg(feature = "declare")]
pub mod declare {
    use super::*;

    asset_file!("../assets.yml");
}
