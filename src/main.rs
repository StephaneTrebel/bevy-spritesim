use std::u64::MAX as MAX_u64;
use std::{ops::Range, u32::MAX as MAX_u32};

use bevy::{math::vec2, prelude::*, render::camera::ScalingMode, utils::HashMap, window::*};
use bevy_pancam::{PanCam, PanCamPlugin};
use noisy_bevy::simplex_noise_2d_seeded;
use rand::{rngs::StdRng, Rng, SeedableRng};

const WINDOW_PHYSICAL_WIDTH: f32 = 1280.; // In pixels
const WINDOW_PHYSICAL_HEIGHT: f32 = 1280.; // In pixels
const WINDOW_SCALE_FACTOR: f64 = 2.0; // How much tiles are streched out in the beginning
const SPRITE_SIZE: f32 = 16.;
const MAP_WIDTH: i32 = 100;
const MAP_HEIGHT: i32 = 100;

// Better naming for tiles in the tilesheet
const TILESET_INDEX_NORTH_WEST: usize = 0;
const TILESET_INDEX_NORTH: usize = 1;
const TILESET_INDEX_NORTH_EAST: usize = 2;
const TILESET_INDEX_WEST: usize = 5;
const TILESET_INDEX_CENTER_TILE: usize = 6;
const TILESET_INDEX_EAST: usize = 7;
const TILESET_INDEX_SOUTH_WEST: usize = 10;
const TILESET_INDEX_SOUTH: usize = 11;
const TILESET_INDEX_SOUTH_EAST: usize = 12;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Kind {
    Plain,
    Ocean,
    Forest,
}

#[derive(Debug)]
struct Tile {
    kind: Kind,
    transform: Transform,
}

type Map = HashMap<(i32, i32), Tile>;

/// Determines if a tile can be replaced by another
///
/// This is handy when replacing a base Plain tile with a more complex one
/// (like a Forest, Desert, etc.)
///
/// The only tile that is allowed to override this rule is of course the Plain type,
/// since it is placed on Ocean tiles
fn can_replace_continent_tile(kind: &Kind, map: &Map, coordinates: (i32, i32)) -> bool {
    return *kind != Kind::Forest || map.get(&coordinates).unwrap().kind == Kind::Plain;
}

/// Generates a patch of tile at given coordinates.
///
/// This function requires several parameters:
/// - The map and its seed (@TODO Join them in the future ?)
/// - The tile Kind (Forest, Plain, etc.)
/// - The patch radius (its size, since we're making roughly round patches)
/// - Frequency and Amplitude scales are used for roughness (I don't full understand
/// them for the moment...)
fn generate_patch(
    seed: f32,
    map: &mut Map,
    kind: &Kind,
    coordinates: (i32, i32),
    radius: f32,
    frequency_scale: f32,
    amplitude_scale: f32,
) {
    let grid_half_size = radius as i32 + 1;
    for w in -grid_half_size..=grid_half_size {
        for h in -grid_half_size..=grid_half_size {
            let p = vec2(w as f32, h as f32);

            // Compute noise offset (That will contribute to the "blob" shape
            // the patch will have)
            let offset = simplex_noise_2d_seeded(p * frequency_scale, seed) * amplitude_scale;

            // Height will serve, with a threshold cutoff, as sizing the resulting patch
            let height = radius + offset - ((w * w + h * h) as f32).sqrt();
            let min_height = 0.;

            let key = (coordinates.0 + w, coordinates.1 + h);
            if key.0 > 0 && key.1 > 0 && key.0 < MAP_WIDTH && key.1 < MAP_HEIGHT {
                // Only replace tile when necessary (for instance, Forest tiles can only be placed on Plains)
                let replace = can_replace_continent_tile(&kind, map, coordinates);
                if replace && height > min_height {
                    map.insert(
                        key,
                        Tile {
                            kind: kind.clone(),
                            transform: Transform::from_translation(Vec3::new(
                                (coordinates.0 + w) as f32 * SPRITE_SIZE,
                                (coordinates.1 + h) as f32 * SPRITE_SIZE,
                                1., // z-index
                            )),
                        },
                    );
                }
            }
        }
    }
}

/// Generates several patches in one go.
///
/// Use this function to avoid having to place patches one by one.
/// Patches are put in a kinda equidistant positions (based on their count), and
/// every parameter is randomly adjusted to simulate realism and RNG
fn generate_multiple_patches(
    pseudo_rng_instance: &mut StdRng,
    mut map: &mut Map,
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
                pseudo_rng_instance.gen_range(-max_offset..=max_offset)
                    + MAP_WIDTH as i32 * w / count,
                pseudo_rng_instance.gen_range(-max_offset..=max_offset)
                    + MAP_HEIGHT as i32 * h / count,
            ));
        }
    }

    // Main generation process
    for coordinates in patch_centers {
        generate_patch(
            pseudo_rng_instance.gen_range(0..MAX_u32) as f32,
            &mut map,
            &kind,
            coordinates,
            pseudo_rng_instance.gen_range(radius_range.clone()) as f32,
            pseudo_rng_instance.gen_range(frequency_range.clone()),
            pseudo_rng_instance.gen_range(amplitude_range.clone()),
        );
    }
}

/// Main map building function.
///
/// Size are hard-coded so the only need parameter is the PRNG instance to generate
/// seeds for the different layers (patch groups) that are applied on the map.
fn build_map(mut pseudo_rng_instance: &mut StdRng) -> Map {
    let map_seed = pseudo_rng_instance.gen_range(0..MAX_u64);
    dbg!(map_seed);
    let mut map: Map = HashMap::new();

    // Initialize the whole map with Ocean tiles
    {
        for w in 0..MAP_WIDTH {
            for h in 0..MAP_HEIGHT {
                map.insert(
                    (w, h),
                    Tile {
                        kind: Kind::Ocean,
                        transform: Transform::from_translation(Vec3::new(
                            (w as f32) * SPRITE_SIZE,
                            (h as f32) * SPRITE_SIZE,
                            0.,
                        )),
                    },
                );
            }
        }
    }

    // Generate patches of Plain to serve as a main continent
    // (but with an irregular shape by overlapping them)
    generate_multiple_patches(
        &mut pseudo_rng_instance,
        &mut map,
        Kind::Plain,
        6,
        2..20,
        0.04..0.06,
        3.60..4.40,
    );

    // Generate randow patches of Forests
    generate_multiple_patches(
        &mut pseudo_rng_instance,
        &mut map,
        Kind::Forest,
        15,
        1..3,
        0.05..1.0,
        3.60..4.40,
    );

    return map;
}

/// Setup the whole game.
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // PRNG initialization
    let mut pseudo_rng_instance: StdRng = StdRng::from_entropy();
    // Map generation
    let map = build_map(&mut pseudo_rng_instance);

    // Configure Camera that can be panned and zoomed with the mouse
    let mut cam = Camera2dBundle::default();
    cam.transform =
        Transform::from_xyz(WINDOW_PHYSICAL_WIDTH / 2., WINDOW_PHYSICAL_HEIGHT / 2., 0.);
    cam.projection.scaling_mode = ScalingMode::FixedVertical(5000.);
    commands.spawn((cam, PanCam::default()));

    // Load the sprites

    // Tilesets are 5x3-2=13 tiles (1 center tile, 4 edge tiles, 4 corner tiles, and 4 internal corner
    // tiles). There are 2 «dead» (unused) tiles for now.
    let forest_sprite_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("sprites/terrain/forest.png"),
        Vec2::new(SPRITE_SIZE, SPRITE_SIZE),
        5,
        3,
        None,
        None,
    ));
    let ocean_sprite_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("sprites/terrain/ocean.png"),
        Vec2::new(SPRITE_SIZE, SPRITE_SIZE),
        5,
        3,
        None,
        None,
    ));

    // Animated tiles are 2x2=4 frames long
    let plain_sprite_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("sprites/terrain/plain.png"),
        Vec2::new(SPRITE_SIZE, SPRITE_SIZE),
        2,
        2,
        None,
        None,
    ));

    // Indices in the tilesheet (TextureAtlas) that are composing the animation
    let animation_indices = AnimationIndices { first: 0, last: 3 };

    // Display the sprites
    for item in map {
        match item.1.kind {
            Kind::Forest => {
                commands.spawn((SpriteSheetBundle {
                    texture_atlas: forest_sprite_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(TILESET_INDEX_CENTER_TILE),
                    transform: item.1.transform,
                    ..default()
                },));
            }
            Kind::Ocean => {
                commands.spawn((SpriteSheetBundle {
                    texture_atlas: ocean_sprite_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(TILESET_INDEX_CENTER_TILE),
                    transform: item.1.transform,
                    ..default()
                },));
            }
            Kind::Plain => {
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: plain_sprite_atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(animation_indices.clone().first),
                        transform: item.1.transform,
                        ..default()
                    },
                    animation_indices.clone(),
                    AnimationTimer(Timer::from_seconds(1., TimerMode::Repeating)),
                ));
            }
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component, Clone)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

/// There we go !
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SpriteSim".into(),
                        position: WindowPosition::Centered(MonitorSelection::Index(1)),
                        resolution: WindowResolution::new(
                            WINDOW_PHYSICAL_WIDTH,
                            WINDOW_PHYSICAL_HEIGHT,
                        )
                        .with_scale_factor_override(WINDOW_SCALE_FACTOR),
                        present_mode: PresentMode::AutoVsync,
                        window_theme: Some(WindowTheme::Dark),
                        window_level: WindowLevel::AlwaysOnTop,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            GamePlugin,
            PanCamPlugin::default(),
        ))
        .add_systems(Update, animate_sprite)
        .run();
}
