use bevy::{prelude::*, reflect::List};

use crate::{map::TiledMap, tilesheet::TileSet};

pub struct TiledPlugin {
    tiled_map_path: String,
    tile_set_path: String,
    img_path: String,
    scale: f32,
    position: Vec2,
    object_insert_func: fn(u32, Entity, &mut Commands),
}

#[derive(Resource)]
pub struct TiledMapPos(Vec2);

#[derive(Resource, Deref, DerefMut)]
pub struct TiledObjectInsertFunc(fn(u32, Entity, &mut Commands));

#[allow(dead_code)]
impl TiledPlugin {
    pub fn from_json(
        tiled_map_path: &str,
        tile_set_path: &str,
        img_path: &str,
        scale: f32,
    ) -> TiledPlugin {
        TiledPlugin {
            tile_set_path: tile_set_path.to_string(),
            tiled_map_path: tiled_map_path.to_string(),
            img_path: img_path.to_string(),
            position: Vec2::default(),
            scale,
            object_insert_func: |_, _, _| {},
        }
    }
    pub fn set_position(mut self, pos: Vec2) -> Self {
        self.position = pos;
        self
    }
    pub fn set_object_insert_func(
        mut self,
        func: fn(id: u32, entity: Entity, commands: &mut Commands),
    ) -> Self {
        self.object_insert_func = func;
        self
    }
}

#[derive(Resource, Deref)]
pub struct TiledMapScale(pub f32);

impl Plugin for TiledPlugin {
    fn build(&self, app: &mut App) {
        let map = TiledMap::from_json(&self.tiled_map_path);
        let mut tile_set = TileSet::from_json(&self.tile_set_path);
        tile_set.image = self.img_path.clone();
        app.insert_resource(TiledObjectInsertFunc(self.object_insert_func))
            .insert_resource(TiledMapPos(self.position))
            .insert_resource(TiledMapScale(self.scale))
            .insert_resource(map)
            .insert_resource(tile_set)
            .add_systems(Startup, (init_asset.before(spawn_world), spawn_world));
    }
}

pub fn spawn_world(
    mut commands: Commands,
    map: Res<TiledMap>,
    tiles_image: Res<TilesImage>,
    scale: Res<TiledMapScale>,
    pos: Res<TiledMapPos>,
    tiled_object_insert_func: Res<TiledObjectInsertFunc>,
) {
    map.layers
        .iter()
        .enumerate()
        .for_each(|(layer_index, layer)| {
            let layer_name = layer.name.as_str();
            match layer_name {
                "collision" => match &layer.data {
                    Some(data) => {
                        data.iter().enumerate().for_each(|(index, tile)| {
                            if let Some(_) = allow_tile(tile) {
                                commands.spawn(spawner(
                                    &map,
                                    pos.0,
                                    scale.0,
                                    index,
                                    |col, row, xoffset, yoffset| {
                                        spawn_collision(col, row, xoffset, yoffset, layer_index)
                                    },
                                ));
                            }
                        });
                    }
                    None => {}
                },
                "objects" => match &layer.objects {
                    Some(objects) => {
                        objects.iter().for_each(|object| {
                            let bundle = (SpriteBundle {
                                transform: Transform::from_xyz(
                                    object.x * 1.9131,
                                    -object.y * 1.8749,
                                    10.,
                                ),
                                sprite: Sprite {
                                    color: Color::srgb(255., 0., 0.),
                                    custom_size: Some(Vec2::new(10., 10.)),
                                    ..default()
                                },
                                ..default()
                            },);
                            let entity = commands.spawn(bundle).id();
                            (tiled_object_insert_func.0)(object.id, entity, &mut commands);
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
                                    pos.0,
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
                                            layer_index,
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
    layer_index: usize,
) -> impl Bundle {
    let mut atlas = tiles_image.atlas.to_owned();
    atlas.index = tile as usize;
    let sprite_bundle = SpriteBundle {
        texture: tiles_image.texture.to_owned(),
        transform: Transform::from_xyz(col, row, layer_index as f32),
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

fn spawn_collision(
    col: f32,
    row: f32,
    xoffset: f32,
    yoffset: f32,
    layer_index: usize,
) -> impl Bundle {
    (
        TiledCollisionSize(Vec2::new(xoffset, yoffset)),
        TransformBundle::from_transform(Transform::from_xyz(col, row, layer_index as f32)),
    )
}

fn spawner<F, B>(map: &Res<TiledMap>, pos: Vec2, scale: f32, index: usize, func: F) -> B
where
    F: Fn(f32, f32, f32, f32) -> B,
    B: Bundle,
{
    let xoffset = map.tilewidth * scale;
    let yoffset = map.tileheight * scale;
    let col = ((index % map.width) as f32 * xoffset) - pos.x;
    let row = -((index / map.width) as f32 * yoffset) - pos.y;
    func(col, row, xoffset, yoffset)
}

fn allow_tile(tile: &dyn Reflect) -> Option<i32> {
    let tile = *tile.downcast_ref::<i32>().unwrap();
    if tile == 0 {
        return None;
    }
    return Some(tile - 1);
}
