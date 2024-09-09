use bevy::{prelude::*, reflect::List};

use crate::{map::TiledMap, tilesheet::TileSet};

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
            .add_systems(Startup, (init_asset.before(spawn_world), spawn_world));
    }
}

#[derive(Resource)]
struct TilesImage {
    atlas: TextureAtlas,
    texture: Handle<Image>,
}

fn init_asset(mut commands: Commands, server: Res<AssetServer>, tileset: Res<TileSet>) {
    let image: Handle<Image> = server.load(&tileset.image);
    let atlas = TextureAtlas {
        layout: server.add(TextureAtlasLayout::from_grid(
            UVec2::new(tileset.tilewidth, tileset.tileheight),
            tileset.columns,
            tileset.tilecount / tileset.columns,
            None,
            None,
        )),
        index: 0,
    };

    commands.insert_resource(TilesImage {
        atlas,
        texture: image,
    });
}

fn create_tile_bundle(
    tile: usize,
    tiles_image: &Res<TilesImage>,
    col: u32,
    row: u32,
    xoffset: f32,
    yoffset: f32,
) -> impl Bundle {
    let mut atlas = tiles_image.atlas.to_owned();
    atlas.index = tile as usize;
    (
        SpriteBundle {
            texture: tiles_image.texture.to_owned(),
            transform: Transform::from_xyz(col as f32 * xoffset, -(row as f32 * yoffset), 0.0),
            ..default()
        },
        atlas,
    )
}

fn spawn_tile(
    tile: i32,
    commands: &mut Commands,
    map: &Res<TiledMap>,
    tiles_image: &Res<TilesImage>,
    index: i32,
) {
    if tile == 0 {
        return;
    }

    let col = index as u32 % map.width;
    let row = index as u32 / map.width;

    commands.spawn(create_tile_bundle(
        tile as usize - 1,
        &tiles_image,
        col,
        row,
        map.tilewidth as f32,
        map.tileheight as f32,
    ));
}

#[derive(Component, Deref, DerefMut)]
pub struct TiledCollisionSize(Vec2);

fn spawn_collision(commands: &mut Commands, map: &Res<TiledMap>, index: i32) {
    let col = index as u32 % map.width;
    let row = index as u32 / map.width;
    commands.spawn((
        Transform::from_xyz(
            col as f32 * map.tilewidth as f32,
            -(row as f32 * map.tileheight as f32),
            0.0,
        ),
        GlobalTransform::default(),
        TiledCollisionSize(Vec2::new(map.tilewidth as f32, map.tileheight as f32)),
    ));
}
fn spawn_world(mut commands: Commands, map: Res<TiledMap>, tiles_image: Res<TilesImage>) {
    map.layers.iter().for_each(|layer| {
        let mut index = 0;
        let layer_name = layer.name.as_str();
        match layer_name {
            "collision" => {
                layer.data.iter().for_each(|_| {
                    spawn_collision(&mut commands, &map, index);
                    index += 1;
                });
            }
            _ => {
                layer.data.iter().for_each(|tile| {
                    let tile = *tile.downcast_ref::<i32>().unwrap();
                    spawn_tile(tile, &mut commands, &map, &tiles_image, index);
                    index += 1;
                });
            }
        }
    });
}
