use bevy::{
    app::{App, Startup},
    prelude::*,
    DefaultPlugins,
};
use debug_plugin::TiledDebugPlugin;
use plugin::TiledPlugin;

mod debug_plugin;
mod map;
mod plugin;
mod prelude;
mod tilesheet;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TiledPlugin::from_json("assets/map.json", "assets/tiles.json"),
            TiledDebugPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let cam = Camera2dBundle::default();
    commands.spawn(cam);
}
