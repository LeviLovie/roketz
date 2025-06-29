use rasset::prelude::*;

asset_def! {
    Terrain: {
        texture: Vec<u8>,
        map: Vec<u8>,
        player_one_x: i64,
        player_one_y: i64,
        player_two_x: i64,
        player_two_y: i64,
        kill_distance: i64,
    },
}

pub fn registry(binary: Vec<u8>) -> Result<Registry, Error> {
    Registry::builder().reg_type::<Terrain>().load(&binary)
}

#[cfg(feature = "declare")]
pub mod declare {
    use super::*;

    asset_file!("../../assets/assets.ron");
}
