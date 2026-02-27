use bevy::prelude::*;

use crate::plugins::{SpriteAtlas, map::MapResource};

pub fn draw_map(mut commands: Commands, atlas: Res<SpriteAtlas>, map: Res<MapResource>) {
    info!("Drawing map…");
    for (_map_coordinates, tile) in map.map.iter() {
        for (_layer, &(kind, variant)) in tile.layers.iter() {
            commands.spawn((
                atlas.sprite(&kind.get_sprite_type(), variant),
                Transform {
                    translation: Vec3::new(tile.real_coordinates.0, tile.real_coordinates.1, 0.),
                    scale: Vec3::splat(1.),
                    ..default()
                },
            ));
        }
    }
}
