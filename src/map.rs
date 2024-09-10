use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TiledMapSet {
    pub firstgid: u32,
    pub source: String,
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
