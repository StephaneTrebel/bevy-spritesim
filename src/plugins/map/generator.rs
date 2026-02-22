use bevy::platform::collections::HashMap;
use bevy::{math::vec2, prelude::*};
use noisy_bevy::{fbm_simplex_2d, simplex_noise_2d_seeded};
use rand::SeedableRng;
use rand::{Rng, rngs::StdRng};
use std::ops::Range;

use crate::plugins::map::{Kind, Layer, Map, Tile, TileLayers, get_layer_from_kind};
use crate::plugins::{BiomeType, SPRITE_SIZE, SpecialType, TerrainType};

const MAP_HEIGHT: i32 = 200;
const MAP_WIDTH: i32 = 200;

/// Generates several terrain patches in one go.
///
/// Use this function to avoid having to place patches one by one.
/// Patches are put in a kinda equidistant positions (based on their count), and
/// every parameter is randomly adjusted to simulate realism and RNG
fn generate_multiple_patches(
    pseudo_rng_instance: &mut StdRng,
    map: &mut Map,
    kind: Kind,
    count: i32,
    radius_range: Range<i32>,
    frequency_range: Range<f32>,
    amplitude_range: Range<f32>,
) {
    // Positions patches centers on the map
    // (kinda equidistant, but with random variations)
    let max_offset = 5;
    let mut patch_centers: Vec<(i32, i32)> = Vec::new();
    for w in 1..count {
        for h in 1..count {
            patch_centers.push((
                pseudo_rng_instance.random_range(-max_offset..=max_offset) + MAP_WIDTH * w / count,
                pseudo_rng_instance.random_range(-max_offset..=max_offset) + MAP_HEIGHT * h / count,
            ));
        }
    }

    // Main generation process
    for coordinates in patch_centers {
        let radius = pseudo_rng_instance.random_range(radius_range.clone()) as f32;
        let frequency_scale = pseudo_rng_instance.random_range(frequency_range.clone());
        let amplitude_scale = pseudo_rng_instance.random_range(amplitude_range.clone());
        let grid_half_size = radius as i32 + 1;
        for w in -grid_half_size..=grid_half_size {
            for h in -grid_half_size..=grid_half_size {
                // Compute noise offset (That will contribute to the "blob" shape
                // the patch will have)
                let offset = simplex_noise_2d_seeded(
                    vec2(w as f32, h as f32) * frequency_scale,
                    pseudo_rng_instance.random_range(0..u32::MAX) as f32,
                ) * amplitude_scale;

                // Height will serve, with a threshold cutoff, as sizing the resulting patch
                let height = radius + offset - ((w * w + h * h) as f32).sqrt();
                let height_threshold = 0.;

                let map_coordinates = (
                    // No sense in adding tiles outside of the map
                    (coordinates.0 + w).clamp(1, MAP_WIDTH - 1),
                    (coordinates.1 + h).clamp(1, MAP_HEIGHT - 1),
                );

                let layers = map.get(&map_coordinates).unwrap().layers.clone();

                // Here we go !
                if
                // Height threshold for size the shape
                (height > height_threshold) &&
                // Only replace tile when necessary (for instance, Forest tiles can only be placed on Plains)
                ( kind != Kind::Biome(BiomeType::Forest)
                  || ( layers.get(&Layer::Terrain).unwrap().0
                       == Kind::Terrain(TerrainType::Plain) ) && layers.get(&Layer::Biome).is_none())
                {
                    let screen_coordinates = (
                        map_coordinates.0 as f32 * SPRITE_SIZE,
                        map_coordinates.1 as f32 * SPRITE_SIZE,
                    );
                    let mut existing_tile_layers = layers.clone();

                    // @TODO Hack for regular terrain generation, should be better handled
                    existing_tile_layers.remove(&Layer::Biome);

                    let layer = get_layer_from_kind(&kind);
                    map.insert(map_coordinates, {
                        existing_tile_layers.insert(
                            layer,
                            (
                                kind,
                                get_tile_layer_variant(&Some(kind), map, &map_coordinates, layer),
                            ),
                        );
                        Tile {
                            layers: existing_tile_layers,
                            real_coordinates: screen_coordinates,
                        }
                    });
                }
            }
        }
    }
}

/// Only used in building the map
fn update_tile_in_map(
    map: &mut Map,
    map_coordinates: &(i32, i32),
    terrain_kind: Option<&TerrainType>,
    biome_kind: Option<&BiomeType>,
    special_kind: Option<&SpecialType>,
) {
    map.insert(*map_coordinates, {
        Tile {
            layers: {
                let mut layers = match map.get(map_coordinates) {
                    Some(tile) => tile.layers.clone(),
                    None => TileLayers::new(),
                };
                if let Some(kind) = terrain_kind {
                    let layer = Layer::Terrain;
                    let kind = Kind::Terrain(*kind);
                    layers.insert(
                        layer,
                        (
                            kind,
                            get_tile_layer_variant(&Some(kind), map, map_coordinates, layer),
                        ),
                    );
                };
                if let Some(kind) = biome_kind {
                    let layer = Layer::Biome;
                    let kind = Kind::Biome(*kind);
                    layers.insert(
                        layer,
                        (
                            kind,
                            get_tile_layer_variant(&Some(kind), map, map_coordinates, layer),
                        ),
                    );
                };
                if let Some(kind) = special_kind {
                    let layer = Layer::Terrain;
                    let kind = Kind::Special(*kind);
                    layers.insert(
                        layer,
                        (
                            kind,
                            get_tile_layer_variant(&Some(kind), map, map_coordinates, layer),
                        ),
                    );
                };
                layers
            },
            real_coordinates: (
                (map_coordinates.0 as f32) * SPRITE_SIZE,
                (map_coordinates.1 as f32) * SPRITE_SIZE,
            ),
        }
    });
}

/// Retrieve the related layer of a Kind
pub fn get_kind_of_tile_layer(tile: &Tile, layer: &Layer) -> Option<Kind> {
    tile.layers.get(layer).map(|l| l.0)
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
pub fn get_tile_layer_variant(
    kind: &Option<Kind>,
    map: &Map,
    map_coordinates: &(i32, i32),
    layer: Layer,
) -> u8 {
    let default_kind = Kind::Terrain(TerrainType::Debug);

    // TODO Use _base_tile ? It was previously use for the underlying tile
    let (variant, _base_tile) = match layer {
        l if l == Layer::Terrain
            || l == Layer::Biome
            || l == Layer::Special && *kind == Some(Kind::Special(SpecialType::Ore)) =>
        {
            let top_left = map
                .get(&(map_coordinates.0 - 1, map_coordinates.1 + 1))
                .and_then(|tile| get_kind_of_tile_layer(tile, &layer))
                .or(Some(default_kind));
            let top = map
                .get(&(map_coordinates.0, map_coordinates.1 + 1))
                .and_then(|tile| get_kind_of_tile_layer(tile, &layer))
                .or(Some(default_kind));
            let top_right = map
                .get(&(map_coordinates.0 + 1, map_coordinates.1 + 1))
                .and_then(|tile| get_kind_of_tile_layer(tile, &layer))
                .or(Some(default_kind));
            let left = map
                .get(&(map_coordinates.0 - 1, map_coordinates.1))
                .and_then(|tile| get_kind_of_tile_layer(tile, &layer))
                .or(Some(default_kind));
            let right = map
                .get(&(map_coordinates.0 + 1, map_coordinates.1))
                .and_then(|tile| get_kind_of_tile_layer(tile, &layer))
                .or(Some(default_kind));
            let bottom_left = map
                .get(&(map_coordinates.0 - 1, map_coordinates.1 - 1))
                .and_then(|tile| get_kind_of_tile_layer(tile, &layer))
                .or(Some(default_kind));
            let bottom = map
                .get(&(map_coordinates.0, map_coordinates.1 - 1))
                .and_then(|tile| get_kind_of_tile_layer(tile, &layer))
                .or(Some(default_kind));
            let bottom_right = map
                .get(&(map_coordinates.0 + 1, map_coordinates.1 - 1))
                .and_then(|tile| get_kind_of_tile_layer(tile, &layer))
                .or(Some(default_kind));

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
            // or Some(kind) which is the "background" tile on top of which a partial tile
            // will be applied (think an ocean shore on top of a plain to make a beach).
            match (
                top_left == *kind,
                top == *kind,
                top_right == *kind,
                left == *kind,
                right == *kind,
                bottom_left == *kind,
                bottom == *kind,
                bottom_right == *kind,
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
        }
        _ => (0, None),
    };

    variant
}

/// Main map generation function.
///
/// Size are hard-coded so the only need parameter is the PRNG instance to generate
/// seeds for the different layers (patch groups) that are applied on the map.
pub fn generate_map() -> Map {
    let mut pseudo_rng_instance = StdRng::from_rng(&mut rand::rng());
    let map_seed = pseudo_rng_instance.random_range(0..u64::MAX);
    dbg!(map_seed);
    let mut map: Map = HashMap::new();

    // Noise map parameters
    let frequency_scale: f32 = pseudo_rng_instance.random_range(0.03..0.06);
    let amplitude_scale: f32 = pseudo_rng_instance.random_range(100.0..130.0);
    let octaves: usize = pseudo_rng_instance.random_range(5..15);
    let lacunarity: f32 = pseudo_rng_instance.random_range(1.8..2.0);
    let gain: f32 = pseudo_rng_instance.random_range(0.5..0.6);

    let map_middle_h = MAP_HEIGHT / 2;

    // Initialize the whole map terrains
    for w in 0..=MAP_WIDTH {
        for h in 0..=MAP_HEIGHT {
            let p = vec2(w as f32, h as f32);
            let offset = fbm_simplex_2d(
                p * frequency_scale,
                octaves,
                lacunarity,
                gain,
                // map_seed as f32,
            ) * amplitude_scale
                * 0.015;

            // For regular terrain tiles, we will check their latitude and use
            // the appropriate terrain type to simulate the earth distribution.
            let base_terrain = {
                let desert_band_thickness =
                    pseudo_rng_instance.random_range(5 * MAP_HEIGHT / 100..10 * MAP_HEIGHT / 100);
                let delta = pseudo_rng_instance.random_range(0..10 * MAP_HEIGHT / 100);
                if h > map_middle_h - desert_band_thickness - delta
                    && h < map_middle_h + desert_band_thickness + delta
                {
                    TerrainType::Desert
                } else {
                    TerrainType::Plain
                }
            };

            let plain_threshold = 0.;
            let hill_threshold = 1.3;
            let mountain_threshold = 1.8;

            // Depending on the offset (the point "height" in the noise map),
            // we will have either an Ocean tile or a regular terrain tile.
            match offset {
                o if o >= plain_threshold && o < hill_threshold => {
                    update_tile_in_map(&mut map, &(w, h), Some(&base_terrain), None, None);
                }
                o if o >= hill_threshold && o < mountain_threshold => {
                    update_tile_in_map(
                        &mut map,
                        &(w, h),
                        Some(&base_terrain),
                        Some(&BiomeType::Hill),
                        None,
                    );
                }
                o if o >= mountain_threshold => {
                    update_tile_in_map(
                        &mut map,
                        &(w, h),
                        Some(&base_terrain),
                        Some(&BiomeType::Hill),
                        Some(&SpecialType::Ore),
                    );
                }
                _ => {
                    update_tile_in_map(
                        &mut map,
                        &(w, h),
                        Some(&base_terrain),
                        Some(&BiomeType::Ocean),
                        None,
                    );
                }
            }
        }
    }

    //    Generate random patches of Forests
    generate_multiple_patches(
        &mut pseudo_rng_instance,
        &mut map,
        Kind::Biome(BiomeType::Forest),
        15,
        1..3,
        0.05..1.0,
        3.60..4.40,
    );

    // Place specials
    for w in 0..=MAP_WIDTH {
        for h in 0..=MAP_HEIGHT {
            let tile = map.get(&(w, h)).unwrap();
            let terrain_kind = tile.layers.get(&Layer::Terrain).unwrap();
            let feature_kind = tile.layers.get(&Layer::Biome);
            // let special_kind = tile.layers.get(&Layer::Special);
            match (w, h) {
                // Corn goes on feature-less plains
                (w, h)
                    if terrain_kind.0 == Kind::Terrain(TerrainType::Plain)
                        && feature_kind.is_none()
                        && pseudo_rng_instance.random_bool(0.01) =>
                {
                    update_tile_in_map(&mut map, &(w, h), None, None, Some(&SpecialType::Corn))
                }
                // Lumber goes on forests
                (w, h)
                    if feature_kind.is_some_and(|k| k.0 == Kind::Biome(BiomeType::Forest))
                        && pseudo_rng_instance.random_bool(0.05) =>
                {
                    update_tile_in_map(&mut map, &(w, h), None, None, Some(&SpecialType::Lumber))
                }
                // Fish goes on oceans
                (w, h)
                    if feature_kind.is_some_and(|k| k.0 == Kind::Biome(BiomeType::Ocean))
                        && pseudo_rng_instance.random_bool(0.01) =>
                {
                    update_tile_in_map(&mut map, &(w, h), None, None, Some(&SpecialType::Fish))
                }
                _ => {}
            }
        }
    }

    map
}
