use rasset::prelude::*;

asset_def! {
    Shader: {
        vertex: Vec<u8>,
        fragment: Vec<u8>,
    },
    Texture: {
        width: i64,
        height: i64,
        texture: Vec<u8>,
    }
}

pub fn registry(binary: Vec<u8>) -> Result<Registry, Error> {
    Registry::builder()
        .reg_type::<Shader>()
        .reg_type::<Texture>()
        .load(&binary)
}

#[cfg(feature = "declare")]
pub mod declare {
    use super::*;

    asset_file!("../assets/assets.ron");
}
