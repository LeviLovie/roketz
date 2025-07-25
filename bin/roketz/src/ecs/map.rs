use anyhow::{Result, bail};
use bevy_ecs::prelude::*;
use rdss::Loader;
use std::sync::{Arc, Mutex};

use crate::ecs::r::Assets;

#[derive(knus::Decode, Debug)]
pub struct MapEntry {
    #[knus(argument)]
    pub path: String,
}

pub fn get_maps(assets: &mut ResMut<Assets>) -> Result<Vec<MapEntry>> {
    let maps_file = assets.borrow().read("maps/maps.kdl")?;
    let maps = knus::parse::<Vec<MapEntry>>("maps/maps.kdl", &maps_file)
        .map_err(|e| anyhow::anyhow!("Failed to parse maps: {}", e))?;
    Ok(maps)
}

pub fn get_maps_raw(assets: &mut Arc<Mutex<Loader>>) -> Result<Vec<String>> {
    let mut maps_file = match assets.lock() {
        Ok(loader) => loader,
        Err(e) => bail!("Failed to read maps file: {}", e),
    };
    let maps_contents = maps_file.read("maps/maps.kdl")?;
    let maps: Vec<MapEntry> = knus::parse("maps/maps.kdl", &maps_contents)
        .map_err(|e| anyhow::anyhow!("Failed to parse maps: {}", e))?;
    Ok(maps.into_iter().map(|m| m.path).collect())
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

pub fn get_map(assets: &mut ResMut<Assets>, name: &str) -> Result<Map> {
    let map_dir = format!("maps/{name}");
    let map_kdl_path = format!("{map_dir}/map.kdl");
    let map_kdl = assets
        .borrow()
        .read(&map_kdl_path)
        .map_err(|e| anyhow::anyhow!("Failed to read map file {}: {}", map_kdl_path, e))?;
    let map = knus::parse::<Map>(&map_kdl_path, &map_kdl)
        .map_err(|e| anyhow::anyhow!("Failed to parse map: {}", e))?;
    Ok(map)
}

pub fn get_map_raw(assets: &mut Arc<Mutex<Loader>>, name: &str) -> Result<Map> {
    let map_dir = format!("maps/{name}");
    let map_kdl_path = format!("{map_dir}/map.kdl");
    let mut maps_file = match assets.lock() {
        Ok(loader) => loader,
        Err(e) => bail!("Failed to read map file {}: {}", map_kdl_path, e),
    };
    let map_kdl = maps_file.read(&map_kdl_path)?;
    let map = knus::parse::<Map>(&map_kdl_path, &map_kdl)
        .map_err(|e| anyhow::anyhow!("Failed to parse map: {}", e))?;
    Ok(map)
}
