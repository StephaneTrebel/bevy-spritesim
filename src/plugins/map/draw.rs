use bevy::prelude::*;

use crate::plugins::{
    SpriteAtlas, TerrainLayer,
    map::{Map, MapResource, Tile},
};

pub fn draw_map(mut commands: Commands, atlas: Res<SpriteAtlas>, map_resource: Res<MapResource>) {
    info!("Drawing map…");
    let map = &map_resource.map;
    let full_tile_variant = 8;
    for (map_coordinates, tile) in map.iter() {
        let (variant, base_tile) = get_terrain_variant(tile, map, map_coordinates);
        commands.spawn((
            atlas.sprite(&base_tile.get_sprite_type(), full_tile_variant),
            Transform {
                translation: Vec3::new(tile.real_coordinates.0, tile.real_coordinates.1, 0.),
                scale: Vec3::splat(1.),
                ..default()
            },
        ));
        commands.spawn((
            atlas.sprite(&tile.terrain.get_sprite_type(), variant),
            Transform {
                translation: Vec3::new(tile.real_coordinates.0, tile.real_coordinates.1, 0.),
                scale: Vec3::splat(1.),
                ..default()
            },
        ));
        if let Some(zone) = tile.zone {
            let (variant, base_tile) = get_zone_variant(tile, map, map_coordinates);
            commands.spawn((
                atlas.sprite(&base_tile.get_sprite_type(), full_tile_variant),
                Transform {
                    translation: Vec3::new(tile.real_coordinates.0, tile.real_coordinates.1, 0.),
                    scale: Vec3::splat(1.),
                    ..default()
                },
            ));
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

/// Retrieve the adequate tileset indices to properly display a tile.
///
/// Indeed, tiles can either be one in the center of a patch (hence the tileable
/// center tile will be used), or on the edge (maybe even in a corner), so a proper
/// algorithmic pass must done to ensure the proper tile is used.
///
/// Additionnaly if a «partial» tile (like a corner) is used, we have to add
/// an underlying tile to serve as background so for instance a beach is composed of
/// a plain (its shore) and the ocean (its beach) over it.
pub fn get_terrain_variant(
    tile: &Tile,
    map: &Map,
    map_coordinates: &(u16, u16),
) -> (u8, TerrainLayer) {
    let terrain = tile.terrain;

    // TODO Use _base_tile ? It was previously use for the underlying tile
    let (variant, base_tile) = {
        let Neighbours {
            bottom,
            bottom_left,
            bottom_right,
            left,
            right,
            top,
            top_left,
            top_right,
        } = get_neighbours(map, map_coordinates);

        // The main algorithm relies on a truth table which determines a tileset index
        // to use based on the ones surrounding the current tile:
        //
        // top_left    | top      | top_right
        // left        | OUR TILE | right
        // bottom_left | bottom   | bottom_right
        //
        // Depending on the surround tile we use one of the 47 possible tiles which
        // encompass all possible arrangements of corners, edgeds, internal corners, etc.
        //
        // A second value is returned, which is either None (for regular «full» tiles),
        // or Some(terrain) which is the "background" tile on top of which a partial tile
        // will be applied (think an ocean shore on top of a plain to make a beach).
        match (
            top_left.terrain == terrain,
            top.terrain == terrain,
            top_right.terrain == terrain,
            left.terrain == terrain,
            right.terrain == terrain,
            bottom_left.terrain == terrain,
            bottom.terrain == terrain,
            bottom_right.terrain == terrain,
        ) {
            // Regular corners
            (_, false, _, false, true, _, true, true) => (0, top),
            (_, false, _, true, false, true, true, _) => (2, top),
            (_, true, true, false, true, _, false, _) => (14, left),
            (true, true, _, true, false, _, false, _) => (16, right),

            // Regular sides
            (_, true, true, false, true, _, true, true) => (7, left),
            (true, true, _, true, false, true, true, _) => (9, right),
            (_, false, _, true, true, true, true, true) => (1, top),
            (true, true, true, true, true, _, false, _) => (15, bottom),

            // 1-width tiles (with edges on either side)
            // Vertical
            (_, false, _, false, false, _, true, _) => (3, top),
            (_, true, _, false, false, _, true, _) => (10, left),
            (_, true, _, false, false, _, false, _) => (17, right),
            // Horizontal
            (_, false, _, false, true, _, false, _) => (21, top),
            (_, false, _, true, true, _, false, _) => (22, top),
            (_, false, _, true, false, _, false, _) => (23, top),

            // Single internal corners (without edges)
            (true, true, true, true, true, true, true, false) => (4, bottom_right),
            (true, true, true, true, true, false, true, true) => (5, bottom_left),
            (true, true, false, true, true, true, true, true) => (11, top_right),
            (false, true, true, true, true, true, true, true) => (12, top_left),

            // Single internal corners (with vertical edges)
            (_, true, true, false, true, _, true, false) => (28, left),
            (true, true, _, true, false, false, true, _) => (29, right),
            (_, true, false, false, true, _, true, true) => (35, top_right),
            (false, true, _, true, false, true, true, _) => (36, top_left),

            // Single internal corners (with horizontal edges)
            (_, false, _, true, true, true, true, false) => (30, top),
            (_, false, _, true, true, false, true, true) => (31, top),
            (true, true, false, true, true, _, false, _) => (37, top_right),
            (false, true, true, true, true, _, false, _) => (38, top_left),

            // Double internal corners (without edges)
            (false, true, false, true, true, true, true, true) => (6, top_left),
            (false, true, true, true, true, false, true, true) => (13, top_left),
            (true, true, false, true, true, true, true, false) => (20, top_right),
            (true, true, true, true, true, false, true, false) => (27, bottom_right),
            (true, true, false, true, true, false, true, true) => (44, top_right),
            (false, true, true, true, true, true, true, false) => (45, top_left),

            // Triple internal corners (without edges)
            (false, true, false, true, true, true, true, false) => (18, top_left),
            (false, true, true, true, true, false, true, false) => (19, top_left),
            (true, true, false, true, true, false, true, false) => (25, top_right),
            (false, true, false, true, true, false, true, true) => (26, top_left),

            // Corners + opposite internal corners
            (_, false, _, false, true, _, true, false) => (32, top),
            (_, false, _, true, false, false, true, _) => (34, top),
            (_, true, false, false, true, _, false, _) => (46, top_right),
            (false, true, _, true, false, _, false, _) => (48, top_left),

            // Edges + opposite internal corners
            (_, false, _, true, true, false, true, false) => (33, top),
            (_, true, false, false, true, _, true, false) => (39, top_right),
            (false, true, _, true, false, false, true, _) => (41, top_left),
            (false, true, false, true, true, _, false, _) => (47, top_left),

            // Center tiles (either isolated, with or without full corners, etc.)
            (true, true, true, true, true, true, true, true) => (8, top_left),
            (false, true, false, true, true, false, true, false) => (40, top_left),
            (_, _, _, _, _, _, _, _) => (24, top), // "top" is always false in the default case
        }
    };

    (variant, base_tile.terrain)
}

pub fn get_zone_variant(
    tile: &Tile,
    map: &Map,
    map_coordinates: &(u16, u16),
) -> (u8, TerrainLayer) {
    let zone = tile.zone;

    // TODO Use _base_tile ? It was previously use for the underlying tile
    let (variant, base_tile) = {
        let Neighbours {
            bottom,
            bottom_left,
            bottom_right,
            left,
            right,
            top,
            top_left,
            top_right,
        } = get_neighbours(map, map_coordinates);

        // The main algorithm relies on a truth table which determines a tileset index
        // to use based on the ones surrounding the current tile:
        //
        // top_left    | top      | top_right
        // left        | OUR TILE | right
        // bottom_left | bottom   | bottom_right
        //
        // Depending on the surround tile we use one of the 47 possible tiles which
        // encompass all possible arrangements of corners, edgeds, internal corners, etc.
        //
        // A second value is returned, which is either None (for regular «full» tiles),
        // or Some(zone) which is the "background" tile on top of which a partial tile
        // will be applied (think an ocean shore on top of a plain to make a beach).
        match (
            top_left.zone == zone,
            top.zone == zone,
            top_right.zone == zone,
            left.zone == zone,
            right.zone == zone,
            bottom_left.zone == zone,
            bottom.zone == zone,
            bottom_right.zone == zone,
        ) {
            // Regular corners
            (_, false, _, false, true, _, true, true) => (0, top),
            (_, false, _, true, false, true, true, _) => (2, top),
            (_, true, true, false, true, _, false, _) => (14, left),
            (true, true, _, true, false, _, false, _) => (16, right),

            // Regular sides
            (_, true, true, false, true, _, true, true) => (7, left),
            (true, true, _, true, false, true, true, _) => (9, right),
            (_, false, _, true, true, true, true, true) => (1, top),
            (true, true, true, true, true, _, false, _) => (15, bottom),

            // 1-width tiles (with edges on either side)
            // Vertical
            (_, false, _, false, false, _, true, _) => (3, top),
            (_, true, _, false, false, _, true, _) => (10, left),
            (_, true, _, false, false, _, false, _) => (17, right),
            // Horizontal
            (_, false, _, false, true, _, false, _) => (21, top),
            (_, false, _, true, true, _, false, _) => (22, top),
            (_, false, _, true, false, _, false, _) => (23, top),

            // Single internal corners (without edges)
            (true, true, true, true, true, true, true, false) => (4, bottom_right),
            (true, true, true, true, true, false, true, true) => (5, bottom_left),
            (true, true, false, true, true, true, true, true) => (11, top_right),
            (false, true, true, true, true, true, true, true) => (12, top_left),

            // Single internal corners (with vertical edges)
            (_, true, true, false, true, _, true, false) => (28, left),
            (true, true, _, true, false, false, true, _) => (29, right),
            (_, true, false, false, true, _, true, true) => (35, top_right),
            (false, true, _, true, false, true, true, _) => (36, top_left),

            // Single internal corners (with horizontal edges)
            (_, false, _, true, true, true, true, false) => (30, top),
            (_, false, _, true, true, false, true, true) => (31, top),
            (true, true, false, true, true, _, false, _) => (37, top_right),
            (false, true, true, true, true, _, false, _) => (38, top_left),

            // Double internal corners (without edges)
            (false, true, false, true, true, true, true, true) => (6, top_left),
            (false, true, true, true, true, false, true, true) => (13, top_left),
            (true, true, false, true, true, true, true, false) => (20, top_right),
            (true, true, true, true, true, false, true, false) => (27, bottom_right),
            (true, true, false, true, true, false, true, true) => (44, top_right),
            (false, true, true, true, true, true, true, false) => (45, top_left),

            // Triple internal corners (without edges)
            (false, true, false, true, true, true, true, false) => (18, top_left),
            (false, true, true, true, true, false, true, false) => (19, top_left),
            (true, true, false, true, true, false, true, false) => (25, top_right),
            (false, true, false, true, true, false, true, true) => (26, top_left),

            // Corners + opposite internal corners
            (_, false, _, false, true, _, true, false) => (32, top),
            (_, false, _, true, false, false, true, _) => (34, top),
            (_, true, false, false, true, _, false, _) => (46, top_right),
            (false, true, _, true, false, _, false, _) => (48, top_left),

            // Edges + opposite internal corners
            (_, false, _, true, true, false, true, false) => (33, top),
            (_, true, false, false, true, _, true, false) => (39, top_right),
            (false, true, _, true, false, false, true, _) => (41, top_left),
            (false, true, false, true, true, _, false, _) => (47, top_left),

            // Center tiles (either isolated, with or without full corners, etc.)
            (true, true, true, true, true, true, true, true) => (8, top_left),
            (false, true, false, true, true, false, true, false) => (40, top_left),
            (_, _, _, _, _, _, _, _) => (24, top), // "top" is always false in the default case
        }
    };

    (variant, base_tile.terrain)
}

struct Neighbours<'a> {
    bottom: &'a Tile,
    bottom_left: &'a Tile,
    bottom_right: &'a Tile,
    left: &'a Tile,
    right: &'a Tile,
    top: &'a Tile,
    top_left: &'a Tile,
    top_right: &'a Tile,
}

fn get_neighbours<'a>(map: &'a Map, map_coordinates: &(u16, u16)) -> Neighbours<'a> {
    let default_tile = &Tile {
        feature: None,
        zone: None,
        terrain: TerrainLayer::Plain,
        real_coordinates: (0., 0.),
    };
    Neighbours {
        top_left: map
            .get(&(map_coordinates.0.saturating_sub(1), map_coordinates.1 + 1))
            .unwrap_or(default_tile),
        top: map
            .get(&(map_coordinates.0, map_coordinates.1 + 1))
            .unwrap_or(default_tile),
        top_right: map
            .get(&(map_coordinates.0 + 1, map_coordinates.1 + 1))
            .unwrap_or(default_tile),
        left: map
            .get(&(map_coordinates.0.saturating_sub(1), map_coordinates.1))
            .unwrap_or(default_tile),
        right: map
            .get(&(map_coordinates.0 + 1, map_coordinates.1))
            .unwrap_or(default_tile),
        bottom_left: map
            .get(&(
                map_coordinates.0.saturating_sub(1),
                map_coordinates.1.saturating_sub(1),
            ))
            .unwrap_or(default_tile),
        bottom: map
            .get(&(map_coordinates.0, map_coordinates.1.saturating_sub(1)))
            .unwrap_or(default_tile),
        bottom_right: map
            .get(&(map_coordinates.0 + 1, map_coordinates.1.saturating_sub(1)))
            .unwrap_or(default_tile),
    }
}
