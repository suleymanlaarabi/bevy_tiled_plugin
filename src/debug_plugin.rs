use bevy::prelude::*;

use crate::default_plugin::TiledCollisionSize;

pub struct TiledDebugPlugin;

impl Plugin for TiledDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, add_sprite_to_collision);
    }
}

fn add_sprite_to_collision(
    mut commands: Commands,
    mut query: Query<(Entity, &TiledCollisionSize, &Transform)>,
) {
    for (entity, collision_size, transform) in query.iter_mut() {
        let bundle = SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0., 0., 255., 0.3),
                custom_size: Some(Vec2::new(collision_size.x, collision_size.y)),
                ..default()
            },
            transform: transform.clone(),
            ..default()
        };
        commands.entity(entity).insert(bundle);
    }
}
