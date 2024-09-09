use bevy::{prelude::*, reflect::List};

use crate::{map::TiledMap, tilesheet::TileSet};

pub struct TiledPlugin {
    tiled_map_path: String,
    tile_set_path: String,
    scale: f32,
}

impl TiledPlugin {
    pub fn from_json(tiled_map_path: &str, tile_set_path: &str, scale: f32) -> TiledPlugin {
        TiledPlugin {
            tile_set_path: tile_set_path.to_string(),
            tiled_map_path: tiled_map_path.to_string(),
            scale,
        }
    }
}

#[derive(Resource, Deref)]
pub struct TiledMapScale(pub f32);

impl Plugin for TiledPlugin {
    fn build(&self, app: &mut App) {
        let map = TiledMap::from_json(&self.tiled_map_path);
        let tile_set = TileSet::from_json(&self.tile_set_path);
        app.insert_resource(TiledMapScale(self.scale))
            .insert_resource(map)
            .insert_resource(tile_set)
            .add_systems(Startup, (init_asset.before(spawn_world), spawn_world));
    }
}

#[derive(Resource)]
pub struct TilesImage {
    pub atlas: TextureAtlas,
    pub texture: Handle<Image>,
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
    col: f32,
    row: f32,
    xoffset: f32,
    yoffset: f32,
) -> impl Bundle {
    let mut atlas = tiles_image.atlas.to_owned();
    atlas.index = tile as usize;
    let sprite_bundle = SpriteBundle {
        texture: tiles_image.texture.to_owned(),
        transform: Transform::from_xyz(col, row, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(xoffset, yoffset)),
            ..default()
        },
        ..default()
    };
    (sprite_bundle, atlas)
}

#[derive(Component, Deref, DerefMut)]
pub struct TiledCollisionSize(Vec2);

fn spawn_collision(col: f32, row: f32, xoffset: f32, yoffset: f32) -> impl Bundle {
    (
        TiledCollisionSize(Vec2::new(xoffset, yoffset)),
        Transform::from_xyz(col, row, 2.),
    )
}

fn spawner<F, B>(map: &Res<TiledMap>, scale: f32, index: usize, func: F) -> B
where
    F: Fn(f32, f32, f32, f32) -> B,
    B: Bundle,
{
    let xoffset = map.tilewidth * scale;
    let yoffset = map.tileheight * scale;
    let col = (index % map.width) as f32 * xoffset;
    let row = -((index / map.width) as f32 * yoffset);
    func(col, row, xoffset, yoffset)
}

fn allow_tile(tile: &dyn Reflect) -> Option<i32> {
    let tile = *tile.downcast_ref::<i32>().unwrap();
    if tile == 0 {
        return None;
    }
    return Some(tile - 1);
}

pub fn spawn_world(
    mut commands: Commands,
    map: Res<TiledMap>,
    tiles_image: Res<TilesImage>,
    scale: Res<TiledMapScale>,
) {
    map.layers.iter().for_each(|layer| {
        let layer_name = layer.name.as_str();
        match layer_name {
            "collision" => match &layer.data {
                Some(data) => {
                    data.iter().enumerate().for_each(|(index, tile)| {
                        if let Some(_) = allow_tile(tile) {
                            commands.spawn(spawner(&map, scale.0, index, spawn_collision));
                        }
                    });
                }
                None => {}
            },
            _ => match &layer.data {
                Some(data) => {
                    data.iter().enumerate().for_each(|(index, tile)| {
                        if let Some(tile) = allow_tile(tile) {
                            commands.spawn(spawner(
                                &map,
                                scale.0,
                                index,
                                |col, row, xoffset, yoffset| {
                                    create_tile_bundle(
                                        tile as usize,
                                        &tiles_image,
                                        col,
                                        row,
                                        xoffset,
                                        yoffset,
                                    )
                                },
                            ));
                        }
                    });
                }
                None => {}
            },
        }
    });
}
