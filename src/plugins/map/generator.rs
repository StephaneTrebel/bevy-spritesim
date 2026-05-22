use bevy::{math::vec2, prelude::*};
use noisy_bevy::{fbm_simplex_2d, simplex_noise_2d_seeded};
use rand::SeedableRng;
use rand::{Rng, rngs::StdRng};
use std::ops::Range;

use crate::plugins::map::{Map, MapCoordinates, Tile};
use crate::plugins::{FeatureLayer, MAP_HEIGHT, MAP_WIDTH, TerrainLayer, ZoneLayer};

/// Generates several terrain patches in one go.
///
/// Use this function to avoid having to place patches one by one.
/// Patches are put in a kinda equidistant positions (based on their count), and
/// every parameter is randomly adjusted to simulate realism and RNG
fn generate_multiple_patches_for_a_zone(
    pseudo_rng_instance: &mut StdRng,
    map: &mut Map,
    zone: ZoneLayer,
    count: u16,
    radius_range: Range<u16>,
    frequency_range: Range<f32>,
    amplitude_range: Range<f32>,
) {
    // Positions patches centers on the map
    // (kinda equidistant, but with random variations)
    let max_offset: i16 = 5;
    let mut patch_centers: Vec<(u16, u16)> = Vec::new();
    for w in 1..count {
        for h in 1..count {
            let pw: i16 = pseudo_rng_instance.random_range(-max_offset..=max_offset);
            let ph: i16 = pseudo_rng_instance.random_range(-max_offset..=max_offset);
            let x: u16 = ((pw + MAP_WIDTH as i16) as u16) * w / count;
            let y: u16 = ((ph + MAP_HEIGHT as i16) as u16) * h / count;
            patch_centers.push((x, y));
        }
    }

    // Main generation process
    for coordinates in patch_centers {
        let radius = pseudo_rng_instance.random_range(radius_range.clone()) as f32;
        let frequency_scale = pseudo_rng_instance.random_range(frequency_range.clone());
        let amplitude_scale = pseudo_rng_instance.random_range(amplitude_range.clone());
        let grid_half_size: i16 = radius as i16 + 1;
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

                let map_coordinates = MapCoordinates(
                    // No sense in adding tiles outside of the map
                    ((coordinates.0 as i16 + w) as u16).clamp(1, MAP_WIDTH - 1),
                    ((coordinates.1 as i16 + h) as u16).clamp(1, MAP_HEIGHT - 1),
                );

                let tile = map.get(&map_coordinates).unwrap();

                // Here we go !
                //
                // Height threshold for size the shape
                if (height > height_threshold) &&
                // Only replace tile when necessary (for instance, Forest tiles can only be placed on Plains)
                (zone != ZoneLayer::Forest || ( tile.terrain == TerrainLayer::Plain ))
                {
                    upsert_tile_in_map(map, &map_coordinates, None, Some(&zone), None);
                }
            }
        }
    }
}

/// Only used while building the map
fn upsert_tile_in_map(
    map: &mut Map,
    map_coordinates: &MapCoordinates,
    terrain: Option<&TerrainLayer>,
    zone: Option<&ZoneLayer>,
    feature: Option<&FeatureLayer>,
) {
    debug!("Upserting tile in map at {map_coordinates:?}");
    let existing_tile = map.get(map_coordinates);

    map.set(map_coordinates, {
        Tile {
            terrain: match terrain {
                Some(t) => *t,
                None => existing_tile.expect("No existing tile").terrain,
            },
            zone: if zone.is_some() {
                zone.copied()
            } else {
                match existing_tile {
                    Some(tile) => tile.zone,
                    None => None,
                }
            },
            feature: if feature.is_some() {
                feature.copied()
            } else {
                match existing_tile {
                    Some(tile) => tile.feature,
                    None => None,
                }
            },
        }
    });
}

/// Main map generation function.
///
/// Size are hard-coded so the only need parameter is the PRNG instance to generate
/// seeds for the different layers (patch groups) that are applied on the map.
pub fn generate_map() -> Map {
    info!("Generating map…");
    let mut pseudo_rng_instance = StdRng::from_rng(&mut rand::rng());
    let map_seed = pseudo_rng_instance.random_range(0..u64::MAX);
    info!("Map seed: {map_seed}");
    let mut map: Map = Map::new();

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
                    TerrainLayer::Desert
                } else {
                    TerrainLayer::Plain
                }
            };

            let plain_threshold = 0.;
            let hill_threshold = 1.3;
            let mountain_threshold = 1.8;

            // Depending on the offset (the point "height" in the noise map),
            // we will have either an Ocean tile or a regular terrain tile.
            match offset {
                o if o >= plain_threshold && o < hill_threshold => {
                    upsert_tile_in_map(
                        &mut map,
                        &MapCoordinates(w, h),
                        Some(&base_terrain),
                        None,
                        None,
                    );
                }
                o if o >= hill_threshold && o < mountain_threshold => {
                    upsert_tile_in_map(
                        &mut map,
                        &MapCoordinates(w, h),
                        Some(&base_terrain),
                        Some(&ZoneLayer::Hill),
                        None,
                    );
                }
                o if o >= mountain_threshold => {
                    upsert_tile_in_map(
                        &mut map,
                        &MapCoordinates(w, h),
                        Some(&base_terrain),
                        Some(&ZoneLayer::Mountain),
                        None,
                    );
                }
                _ => {
                    upsert_tile_in_map(
                        &mut map,
                        &MapCoordinates(w, h),
                        Some(&TerrainLayer::Ocean),
                        None,
                        None,
                    );
                }
            }
        }
    }

    // Generate random patches of Forests
    generate_multiple_patches_for_a_zone(
        &mut pseudo_rng_instance,
        &mut map,
        ZoneLayer::Forest,
        15,
        1..3,
        0.05..1.0,
        3.60..4.40,
    );

    // Place specials
    for w in 0..=MAP_WIDTH {
        for h in 0..=MAP_HEIGHT {
            let tile = map.get(&MapCoordinates(w, h)).unwrap();
            let terrain = tile.terrain;
            let zone = tile.zone;
            let probability = pseudo_rng_instance.random_bool(0.01);
            // Corn goes on feature-less plains
            if terrain == TerrainLayer::Plain && zone.is_none() && probability {
                println!("[{}] Putting Corn at {:?}", terrain, &MapCoordinates(w, h));
                upsert_tile_in_map(
                    &mut map,
                    &MapCoordinates(w, h),
                    None,
                    None,
                    Some(&FeatureLayer::Corn),
                )
            }
            // Lumber goes on forests
            else if zone.is_some_and(|k| k == ZoneLayer::Forest) && probability {
                println!("[{}] Putting Lumber at {:?}", terrain, &(w, h));
                upsert_tile_in_map(
                    &mut map,
                    &MapCoordinates(w, h),
                    None,
                    None,
                    Some(&FeatureLayer::Lumber),
                )
            }
            // Fish goes on oceans
            else if terrain == TerrainLayer::Ocean && probability {
                println!("[{}] Putting Fish at {:?}", terrain, &(w, h));
                upsert_tile_in_map(
                    &mut map,
                    &MapCoordinates(w, h),
                    None,
                    None,
                    Some(&FeatureLayer::Fish),
                )
            }
        }
    }

    info!("Done generating map.");
    map
}
