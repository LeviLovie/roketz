use rasset::prelude::*;

asset_def! {
    Shader: {
        vertex: Vec<u8>,
        fragment: Vec<u8>,
    },
    Terrain: {
        texture: Vec<u8>,
        map: Vec<u8>,
        player_start_x: i64,
        player_start_y: i64,
    },
    Destructions: {
        data: Vec<u8>,
    },
}

pub fn registry(binary: Vec<u8>) -> Result<Registry, Error> {
    Registry::builder()
        .reg_type::<Shader>()
        .reg_type::<Terrain>()
        .reg_type::<Destructions>()
        .load(&binary)
}

#[cfg(feature = "declare")]
pub mod declare {
    use super::*;

    asset_file!("../assets/assets.ron");
}
