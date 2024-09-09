use bevy::{
    app::{App, Startup},
    prelude::{Camera2dBundle, Commands},
    DefaultPlugins,
};
use map::TiledMap;
use plugin::TiledPlugin;
use tilesheet::TileSet;

mod map;
mod plugin;
mod prelude;
mod tilesheet;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TiledPlugin::from_json("assets/map.json", "assets/tiles.json"),
        ))
        .add_systems(Startup, (setup_camera, insert_res))
        .run();
}

fn insert_res(mut commands: Commands) {
    let map = TiledMap::from_json("assets/map.json");
    let tileset = TileSet::from_json("assets/tiles.json");
    commands.insert_resource(map);
    commands.insert_resource(tileset);
}

fn setup_camera(mut commands: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.transform.translation.y -= 200.;
    cam.transform.translation.x += 400.;
    commands.spawn(cam);
}
