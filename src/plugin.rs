use bevy::prelude::*;

use crate::{
    map::{Layer, TiledMap},
    tilesheet::TileSet,
};

pub struct TiledPlugin {
    tiled_map_path: String,
    tile_set_path: String,
}

impl TiledPlugin {
    pub fn from_json(tiled_map_path: &str, tile_set_path: &str) -> TiledPlugin {
        TiledPlugin {
            tile_set_path: tile_set_path.to_string(),
            tiled_map_path: tiled_map_path.to_string(),
        }
    }
}

impl Plugin for TiledPlugin {
    fn build(&self, app: &mut App) {
        let map = TiledMap::from_json(&self.tiled_map_path);
        let tile_set = TileSet::from_json(&self.tile_set_path);
        app.insert_resource(map)
            .insert_resource(tile_set)
            .add_systems(Startup, spawn_world);
    }
}

fn create_tile_bundle(tile: u32, layer: &Layer, map: &TiledMap) -> impl Bundle {
    ()
}

fn spawn_world(mut commands: Commands, map: Res<TiledMap>, tileset: Res<TileSet>) {
    let layer = map.layers.get(0).unwrap();
    let tile = layer.data.get(0).unwrap();
    let bundle = create_tile_bundle(*tile, layer, &map);
    commands.spawn(bundle);
}
