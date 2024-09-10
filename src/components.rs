use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct TiledCollisionSize(pub Vec2);

#[derive(Component)]
pub struct TiledObject {
    pub position: Vec2,
    pub id: u32,
}

impl TiledObject {
    pub fn new(position: Vec2, id: u32) -> Self {
        TiledObject { position, id }
    }
}
