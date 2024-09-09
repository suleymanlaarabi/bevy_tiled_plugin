use std::fs::read_to_string;

use bevy::prelude::*;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Resource, Deserialize, Debug)]
pub struct TileSet {
    pub columns: u32,
    pub image: String,
    pub imageheight: u32,
    pub imagewidth: u32,
    pub margin: u32,
    pub name: String,
    pub spacing: u32,
    pub tilecount: u32,
    pub tiledversion: String,
    pub tileheight: u32,
    pub tilewidth: u32,
    #[serde(rename = "type")]
    pub tilesheet_type: String,
    pub version: String,
}

impl TileSet {
    pub fn from_json(path: &str) -> TileSet {
        serde_json::from_str(&read_to_string(path).expect("File not found"))
            .expect("json parse error")
    }
}
