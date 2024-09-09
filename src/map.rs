use std::fs::read_to_string;

use bevy::prelude::*;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TiledMapSet {
    pub firstgid: u32,
    pub source: String,
}

#[allow(dead_code)]
#[derive(Resource, Deserialize, Debug)]
pub struct TiledMap {
    pub compressionlevel: i32,
    pub height: u32,
    pub infinite: bool,
    pub nextlayerid: u32,
    pub nextobjectid: u32,
    pub orientation: String,
    pub renderorder: String,
    pub tiledversion: String,
    pub tileheight: u32,
    pub tilewidth: u32,
    #[serde(rename = "type")]
    pub map_type: String,
    pub version: String,
    pub width: u32,
    pub layers: Vec<Layer>,
    pub tilesets: Vec<TiledMapSet>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Layer {
    pub data: Option<Vec<i32>>,
    pub id: u32,
    pub name: String,
    pub opacity: f32,
    #[serde(rename = "type")]
    pub tile_type: String,
    pub visible: bool,
    pub x: f32,
    pub y: f32,
}

impl TiledMap {
    pub fn from_json(path: &str) -> TiledMap {
        serde_json::from_str(&read_to_string(path).expect("File not found"))
            .expect("json parse error")
    }
}
