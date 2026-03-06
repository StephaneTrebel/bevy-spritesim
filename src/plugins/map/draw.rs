use bevy::prelude::*;

use crate::plugins::{SpriteAtlas, map::MapResource};

pub fn draw_map(mut commands: Commands, atlas: Res<SpriteAtlas>, map: Res<MapResource>) {
    info!("Drawing map…");
    for (_map_coordinates, tile) in map.map.iter() {
        // @TODO handle variant
        let variant = 8;
        commands.spawn((
            atlas.sprite(&tile.terrain.get_sprite_type(), variant),
            Transform {
                translation: Vec3::new(tile.real_coordinates.0, tile.real_coordinates.1, 0.),
                scale: Vec3::splat(1.),
                ..default()
            },
        ));
        if let Some(zone) = tile.zone {
            commands.spawn((
                atlas.sprite(&zone.get_sprite_type(), variant),
                Transform {
                    translation: Vec3::new(tile.real_coordinates.0, tile.real_coordinates.1, 1.),
                    scale: Vec3::splat(1.),
                    ..default()
                },
            ));
        }
        if let Some(feature) = tile.feature {
            commands.spawn((
                atlas.sprite(&feature.get_sprite_type(), variant),
                Transform {
                    translation: Vec3::new(tile.real_coordinates.0, tile.real_coordinates.1, 2.),
                    scale: Vec3::splat(1.),
                    ..default()
                },
            ));
        }
    }
}
