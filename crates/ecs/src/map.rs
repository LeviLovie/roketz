use anyhow::Result;
use rdss::Loader;

#[derive(knus::Decode, Debug)]
pub struct MapEntry {
    #[knus(argument)]
    pub path: String,
}

pub fn get_maps(assets: &mut Loader) -> Result<Vec<MapEntry>> {
    let maps_file = assets.read("maps/maps.kdl")?;
    let maps = knus::parse::<Vec<MapEntry>>("maps/maps.kdl", &maps_file)
        .map_err(|e| anyhow::anyhow!("Failed to parse maps: {}", e))?;
    Ok(maps)
}

#[derive(knus::Decode, Debug, Clone)]
pub struct Map {
    #[knus(child, unwrap(argument))]
    pub name: String,
    #[knus(child, unwrap(argument))]
    pub texture: String,
    #[knus(child, unwrap(argument))]
    pub map: String,
    #[knus(child)]
    pub spawns: Spawns,
}

#[derive(knus::Decode, Debug, Clone)]
pub struct Spawns {
    #[knus(children(name = "spawn"))]
    pub spawns: Vec<Spawn>,
}

#[derive(knus::Decode, Debug, Clone)]
pub struct Spawn {
    #[knus(property)]
    pub x: u32,
    #[knus(property)]
    pub y: u32,
}

pub fn get_map(assets: &mut Loader, name: &str) -> Result<Map> {
    let map_dir = format!("maps/{name}");
    let map_kdl_path = format!("{map_dir}/map.kdl");
    let map_kdl = assets
        .read(&map_kdl_path)
        .map_err(|e| anyhow::anyhow!("Failed to read map file {}: {}", map_kdl_path, e))?;
    let map = knus::parse::<Map>(&map_kdl_path, &map_kdl)
        .map_err(|e| anyhow::anyhow!("Failed to parse map: {}", e))?;
    Ok(map)
}
