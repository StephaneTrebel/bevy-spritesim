use bevy::prelude::*;

use crate::plugins::{
    SPRITE_DISPLAY_SIZE, SpriteAtlas, TerrainLayer,
    map::{Map, MapResource, Tile},
    select_on_click,
};

#[derive(Component)]
pub struct RealCoordinates {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Component)]
pub struct Settler;

/// Tiny scale factor applied to every tile sprite so that adjacent quads
/// overlap by a sub-pixel amount.  Without this, GPU rasterisation rounding
/// at tile boundaries can leave single-pixel gaps (the classic "tile seam"
/// artifact).  The value is small enough to be invisible but large enough
/// to guarantee coverage at any zoom level.
const TILE_SCALE: f32 = 1.001;

pub fn set_transform_for_real_coordinates(
    mut entity: Single<(Entity, &Settler, &RealCoordinates, &mut Transform)>,
) {
    entity.3.translation = Vec3 {
        x: entity.2.x,
        y: entity.2.y,
        z: 90.,
    };
}

pub fn anchor_camera_to_settler(
    settler: Single<(Entity, &Settler, &RealCoordinates)>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    camera.translation = Vec3 {
        x: settler.2.x,
        y: settler.2.y,
        z: 100.,
    };
}

pub fn draw_map(mut commands: Commands, atlas: Res<SpriteAtlas>, map_resource: Res<MapResource>) {
    info!("Drawing map…");

    let map = &map_resource.map;
    let tile_scale = Vec3::splat(TILE_SCALE);
    for (index, (map_coordinates, tile)) in map.iter().enumerate() {
        let terrain_variant = get_terrain_variant(tile, map, map_coordinates);
        let z: f32 = match tile.terrain {
            TerrainLayer::Debug => 0.,
            TerrainLayer::Desert => 1.,
            TerrainLayer::Plain => 2.,
            TerrainLayer::Ocean => 3.,
        };
        commands
            .spawn((
                atlas.sprite(&tile.terrain.get_sprite_type(), terrain_variant),
                Transform {
                    translation: Vec3::new(
                        tile.real_coordinates.0,
                        tile.real_coordinates.1,
                        z + (index as f32 / 10000.),
                    ),
                    scale: tile_scale,
                    ..default()
                },
                RealCoordinates {
                    x: tile.real_coordinates.0,
                    y: tile.real_coordinates.1,
                },
                Pickable::default(),
            ))
            .observe(select_on_click);

        if let Some(zone) = tile.zone {
            let zone_variant = get_zone_variant(tile, map, map_coordinates);

            commands
                .spawn((
                    atlas.sprite(&zone.get_sprite_type(), zone_variant),
                    Transform {
                        translation: Vec3::new(
                            tile.real_coordinates.0,
                            tile.real_coordinates.1,
                            10.,
                        ),
                        scale: tile_scale,
                        ..default()
                    },
                    RealCoordinates {
                        x: tile.real_coordinates.0,
                        y: tile.real_coordinates.1,
                    },
                    Pickable::default(),
                ))
                .observe(select_on_click);
        }

        if let Some(feature) = tile.feature {
            commands
                .spawn((
                    atlas.sprite(&feature.get_sprite_type(), terrain_variant),
                    Transform {
                        translation: Vec3::new(
                            tile.real_coordinates.0,
                            tile.real_coordinates.1,
                            20.,
                        ),
                        scale: tile_scale,
                        ..default()
                    },
                    RealCoordinates {
                        x: tile.real_coordinates.0,
                        y: tile.real_coordinates.1,
                    },
                    Pickable::default(),
                ))
                .observe(select_on_click);
        }
    }

    // Spawn our first settler !
    // His name is "Michel"
    commands
        .spawn((
            atlas.sprite(&crate::plugins::SpriteType::Settler, 0),
            RealCoordinates {
                x: -50. * SPRITE_DISPLAY_SIZE,
                y: -50. * SPRITE_DISPLAY_SIZE,
            },
            Pickable::default(),
            Settler,
        ))
        .observe(select_on_click);

    info!("Done drawing map…");
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
pub fn get_terrain_variant(tile: &Tile, map: &Map, map_coordinates: &(u16, u16)) -> u8 {
    let terrain = tile.terrain;
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
        (_, false, _, false, true, _, true, true) => 0,
        (_, false, _, true, false, true, true, _) => 2,
        (_, true, true, false, true, _, false, _) => 14,
        (true, true, _, true, false, _, false, _) => 16,

        // Regular sides
        (_, true, true, false, true, _, true, true) => 7,
        (true, true, _, true, false, true, true, _) => 9,
        (_, false, _, true, true, true, true, true) => 1,
        (true, true, true, true, true, _, false, _) => 15,

        // 1-width tiles (with edges on either side)
        // Vertical
        (_, false, _, false, false, _, true, _) => 3,
        (_, true, _, false, false, _, true, _) => 10,
        (_, true, _, false, false, _, false, _) => 17,
        // Horizontal
        (_, false, _, false, true, _, false, _) => 21,
        (_, false, _, true, true, _, false, _) => 22,
        (_, false, _, true, false, _, false, _) => 23,

        // Single internal corners (without edges)
        (true, true, true, true, true, true, true, false) => 4,
        (true, true, true, true, true, false, true, true) => 5,
        (true, true, false, true, true, true, true, true) => 11,
        (false, true, true, true, true, true, true, true) => 12,

        // Single internal corners (with vertical edges)
        (_, true, true, false, true, _, true, false) => 28,
        (true, true, _, true, false, false, true, _) => 29,
        (_, true, false, false, true, _, true, true) => 35,
        (false, true, _, true, false, true, true, _) => 36,

        // Single internal corners (with horizontal edges)
        (_, false, _, true, true, true, true, false) => 30,
        (_, false, _, true, true, false, true, true) => 31,
        (true, true, false, true, true, _, false, _) => 37,
        (false, true, true, true, true, _, false, _) => 38,

        // Double internal corners (without edges)
        (false, true, false, true, true, true, true, true) => 6,
        (false, true, true, true, true, false, true, true) => 13,
        (true, true, false, true, true, true, true, false) => 20,
        (true, true, true, true, true, false, true, false) => 27,
        (true, true, false, true, true, false, true, true) => 44,
        (false, true, true, true, true, true, true, false) => 45,

        // Triple internal corners (without edges)
        (false, true, false, true, true, true, true, false) => 18,
        (false, true, true, true, true, false, true, false) => 19,
        (true, true, false, true, true, false, true, false) => 25,
        (false, true, false, true, true, false, true, true) => 26,

        // Corners + opposite internal corners
        (_, false, _, false, true, _, true, false) => 32,
        (_, false, _, true, false, false, true, _) => 34,
        (_, true, false, false, true, _, false, _) => 46,
        (false, true, _, true, false, _, false, _) => 48,

        // Edges + opposite internal corners
        (_, false, _, true, true, false, true, false) => 33,
        (_, true, false, false, true, _, true, false) => 39,
        (false, true, _, true, false, false, true, _) => 41,
        (false, true, false, true, true, _, false, _) => 47,

        // Center tiles (either isolated, with or without full corners, etc.)
        (true, true, true, true, true, true, true, true) => 8,
        (false, true, false, true, true, false, true, false) => 40,
        (_, _, _, _, _, _, _, _) => 24,
    }
}

pub fn get_zone_variant(tile: &Tile, map: &Map, map_coordinates: &(u16, u16)) -> u8 {
    let zone = tile.zone;

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
        (_, false, _, false, true, _, true, true) => 0,
        (_, false, _, true, false, true, true, _) => 2,
        (_, true, true, false, true, _, false, _) => 14,
        (true, true, _, true, false, _, false, _) => 16,

        // Regular sides
        (_, true, true, false, true, _, true, true) => 7,
        (true, true, _, true, false, true, true, _) => 9,
        (_, false, _, true, true, true, true, true) => 1,
        (true, true, true, true, true, _, false, _) => 15,

        // 1-width tiles (with edges on either side)
        // Vertical
        (_, false, _, false, false, _, true, _) => 3,
        (_, true, _, false, false, _, true, _) => 10,
        (_, true, _, false, false, _, false, _) => 17,
        // Horizontal
        (_, false, _, false, true, _, false, _) => 21,
        (_, false, _, true, true, _, false, _) => 22,
        (_, false, _, true, false, _, false, _) => 23,

        // Single internal corners (without edges)
        (true, true, true, true, true, true, true, false) => 4,
        (true, true, true, true, true, false, true, true) => 5,
        (true, true, false, true, true, true, true, true) => 11,
        (false, true, true, true, true, true, true, true) => 12,

        // Single internal corners (with vertical edges)
        (_, true, true, false, true, _, true, false) => 28,
        (true, true, _, true, false, false, true, _) => 29,
        (_, true, false, false, true, _, true, true) => 35,
        (false, true, _, true, false, true, true, _) => 36,

        // Single internal corners (with horizontal edges)
        (_, false, _, true, true, true, true, false) => 30,
        (_, false, _, true, true, false, true, true) => 31,
        (true, true, false, true, true, _, false, _) => 37,
        (false, true, true, true, true, _, false, _) => 38,

        // Double internal corners (without edges)
        (false, true, false, true, true, true, true, true) => 6,
        (false, true, true, true, true, false, true, true) => 13,
        (true, true, false, true, true, true, true, false) => 20,
        (true, true, true, true, true, false, true, false) => 27,
        (true, true, false, true, true, false, true, true) => 44,
        (false, true, true, true, true, true, true, false) => 45,

        // Triple internal corners (without edges)
        (false, true, false, true, true, true, true, false) => 18,
        (false, true, true, true, true, false, true, false) => 19,
        (true, true, false, true, true, false, true, false) => 25,
        (false, true, false, true, true, false, true, true) => 26,

        // Corners + opposite internal corners
        (_, false, _, false, true, _, true, false) => 32,
        (_, false, _, true, false, false, true, _) => 34,
        (_, true, false, false, true, _, false, _) => 46,
        (false, true, _, true, false, _, false, _) => 48,

        // Edges + opposite internal corners
        (_, false, _, true, true, false, true, false) => 33,
        (_, true, false, false, true, _, true, false) => 39,
        (false, true, _, true, false, false, true, _) => 41,
        (false, true, false, true, true, _, false, _) => 47,

        // Center tiles (either isolated, with or without full corners, etc.)
        (true, true, true, true, true, true, true, true) => 8,
        (false, true, false, true, true, false, true, false) => 40,
        (_, _, _, _, _, _, _, _) => 24,
    }
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
