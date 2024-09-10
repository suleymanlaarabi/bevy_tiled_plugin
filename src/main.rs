use bevy::{
    app::{App, Startup},
    prelude::*,
    DefaultPlugins,
};

use debug_plugin::TiledDebugPlugin;
use default_plugin::TiledPlugin;

mod components;
mod debug_plugin;
mod default_plugin;
mod map;
mod resource;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            TiledPlugin::from_json("assets/map.json", "assets/tiles.json", "Hills.png", 2.)
                .set_object_insert_func(insert_object),
            TiledDebugPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn insert_object(id: u32, entity: Entity, commands: &mut Commands) {
    println!("Inserting object with id {} and entity {:?}", id, entity);
    match id {
        16 => {
            commands.entity(entity).with_children(|children| {
                let bundle = SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0., 255., 0., 1.),
                        custom_size: Some(Vec2::new(50., 50.)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..default()
                };
                children.spawn(bundle);
            });
        }
        _ => {}
    }
}

fn setup_camera(mut commands: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.transform.translation = Vec3::new(355.0, -240.0, 10.0);
    commands.spawn(cam);
}
