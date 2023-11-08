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
const TILESET_WIDTH: usize = 7;
const TILESET_HEIGHT: usize = 7;
const ANIMATION_FRAME_COUNT: usize = 4;
const TIME_BETWEEN_FRAMES: f32 = 2.;
const MAP_WIDTH: i32 = 100;
const MAP_HEIGHT: i32 = 100;

/// A Tile is made of several layers, from bottom to top (only the first one is
/// mandatory, the other are all optional):
/// - A base Terrain (Plain, Ocean, etc.)
/// - a Feature (Forest, Hills, etc.)
/// - a Special characteristic (Food, Ore, Silver, etc.)
/// - a Development (Road, Farmland, etc.)
/// - a Settlement (Village, Fort, etc.)
/// - a Unit (Settler, Canon, etc.) that is moving through it
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Layer {
    Terrain,
    Feature,
}

/// Terrain are the base layers of all tiles
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum TerrainKind {
    Plain,
}

/// Features are natural deposits that adds value to a Terrain
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum FeatureKind {
    Ocean,
    Forest,
}

/// This is a union of all sprites types. Used for using common sprite
/// drawing functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Kind {
    TKind(TerrainKind),
    FKind(FeatureKind),
}

/// In-memory map for all layers of a Tile
type TileLayers = HashMap<Layer, Kind>;

/// A «Tile» is a superposition of several things that will compose the Map.
#[derive(Debug)]
struct Tile {
    layers: TileLayers,

    // These are called «real» coordinates because they are not the coordinates
    // in the map, but rather are the coordinates of where the sprite will be drawn
    real_coordinates: (f32, f32),
}

/// Retrieve the related layer of a Kind
fn get_kind_of_tile_layer(tile: &Tile, layer: Layer) -> Option<&Kind> {
    return tile.layers.get(&layer);
}

/// Retrieve the concrete Kind of a tile on a given Layer
fn get_layer_from_kind(kind: &Kind) -> Layer {
    return match kind {
        Kind::TKind(_) => Layer::Terrain,
        Kind::FKind(_) => Layer::Feature,
    };
}

/// Retrieve the concrete kind of a tile on a given Layer

/// Retrieve the adequate tileset index for the tile
///
/// Indeed, tiles can either be one in the center of a patch (hence the tileable
/// center tile will be used), or on the edge (maybe even in a corner), so a proper
/// algorithmic pass must done to ensure the proper tile is used
fn get_tileset_index(tile: &Tile, map: &Map, coordinates: &(i32, i32), layer: Layer) -> usize {
    let kind = get_kind_of_tile_layer(tile, layer);

    let default_tile = tile.clone();

    let top_left = get_kind_of_tile_layer(
        map.get(&(coordinates.0 - 1, coordinates.1 + 1))
            .unwrap_or(&default_tile),
        layer,
    );
    let top = get_kind_of_tile_layer(
        map.get(&(coordinates.0, coordinates.1 + 1))
            .unwrap_or(&default_tile),
        layer,
    );
    let top_right = get_kind_of_tile_layer(
        map.get(&(coordinates.0 + 1, coordinates.1 + 1))
            .unwrap_or(&default_tile),
        layer,
    );
    let left = get_kind_of_tile_layer(
        map.get(&(coordinates.0 - 1, coordinates.1))
            .unwrap_or(&default_tile),
        layer,
    );
    let right = get_kind_of_tile_layer(
        map.get(&(coordinates.0 + 1, coordinates.1))
            .unwrap_or(&default_tile),
        layer,
    );
    let bottom_left = get_kind_of_tile_layer(
        map.get(&(coordinates.0 - 1, coordinates.1 - 1))
            .unwrap_or(&default_tile),
        layer,
    );
    let bottom = get_kind_of_tile_layer(
        map.get(&(coordinates.0, coordinates.1 - 1))
            .unwrap_or(&default_tile),
        layer,
    );
    let bottom_right = get_kind_of_tile_layer(
        map.get(&(coordinates.0 + 1, coordinates.1 - 1))
            .unwrap_or(&default_tile),
        layer,
    );

    return match (
        top_left == kind,
        top == kind,
        top_right == kind,
        left == kind,
        right == kind,
        bottom_left == kind,
        bottom == kind,
        bottom_right == kind,
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
    };
}

/// In-memory map for all gameplay and render purposes.
/// This is the heart of the game.
type Map = HashMap<(i32, i32), Tile>;

/// In-memory map that ties Kind elements with their corresponding
/// TextureAtlas handle.
type TerrainHandleMap = HashMap<Kind, Handle<TextureAtlas>>;

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
                pseudo_rng_instance.gen_range(-max_offset..=max_offset)
                    + MAP_WIDTH as i32 * w / count,
                pseudo_rng_instance.gen_range(-max_offset..=max_offset)
                    + MAP_HEIGHT as i32 * h / count,
            ));
        }
    }

    // Main generation process
    for coordinates in patch_centers {
        let radius = pseudo_rng_instance.gen_range(radius_range.clone()) as f32;
        let frequency_scale = pseudo_rng_instance.gen_range(frequency_range.clone());
        let amplitude_scale = pseudo_rng_instance.gen_range(amplitude_range.clone());
        let grid_half_size = radius as i32 + 1;
        for w in -grid_half_size..=grid_half_size {
            for h in -grid_half_size..=grid_half_size {
                let p = vec2(w as f32, h as f32);

                // Compute noise offset (That will contribute to the "blob" shape
                // the patch will have)
                let offset = simplex_noise_2d_seeded(
                    p * frequency_scale,
                    pseudo_rng_instance.gen_range(0..MAX_u32) as f32,
                ) * amplitude_scale;

                // Height will serve, with a threshold cutoff, as sizing the resulting patch
                let height = radius + offset - ((w * w + h * h) as f32).sqrt();
                let min_height = 0.;

                let key = (coordinates.0 + w, coordinates.1 + h);

                // Here we go !
                if
                // No sense in adding tiles outside of the map
                ( key.0 > 0 && key.1 > 0 && key.0 < MAP_WIDTH && key.1 < MAP_HEIGHT ) &&
                    // Only replace tile when necessary (for instance, Forest tiles can only be placed on Plains)
                    ( kind != Kind::FKind(FeatureKind::Forest)
                    || map.get(&coordinates).unwrap().layers.get(&Layer::Terrain).unwrap() == &Kind::TKind(TerrainKind::Plain)) &&
                    // Height threshold for size the shape
                    (height > min_height)
                {
                    let real_coordinates = (
                        (coordinates.0 + w) as f32 * SPRITE_SIZE,
                        (coordinates.1 + h) as f32 * SPRITE_SIZE,
                    );
                    let default_tile = {
                        let layers = TileLayers::new();
                        Tile {
                            layers,
                            real_coordinates,
                        }
                    };
                    let mut existing_tile_layers =
                        map.get(&key).unwrap_or(&default_tile).layers.clone();
                    // @TODO Hack for Plain terrain generation, should be better handled
                    existing_tile_layers.remove(&Layer::Feature);
                    map.insert(key, {
                        existing_tile_layers.insert(get_layer_from_kind(&kind), kind);
                        Tile {
                            layers: existing_tile_layers,
                            real_coordinates,
                        }
                    });
                }
            }
        }
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
                map.insert((w, h), {
                    Tile {
                        layers: {
                            let mut layers = TileLayers::new();

                            // @TODO Hack for Ocean partial tiles
                            layers.insert(Layer::Terrain, Kind::TKind(TerrainKind::Plain));

                            layers.insert(Layer::Feature, Kind::FKind(FeatureKind::Ocean));
                            layers
                        },
                        real_coordinates: ((w as f32) * SPRITE_SIZE, (h as f32) * SPRITE_SIZE),
                    }
                });
            }
        }
    }

    // Generate patches of Plain to serve as a main continent
    // (but with an irregular shape by overlapping them)
    generate_multiple_patches(
        &mut pseudo_rng_instance,
        &mut map,
        Kind::TKind(TerrainKind::Plain),
        6,
        2..20,
        0.04..0.06,
        3.60..4.40,
    );

    // Generate randow patches of Forests
    generate_multiple_patches(
        &mut pseudo_rng_instance,
        &mut map,
        Kind::FKind(FeatureKind::Forest),
        15,
        1..3,
        0.05..1.0,
        3.60..4.40,
    );

    return map;
}

fn get_zindex_from_kind(kind: &Kind) -> f32 {
    return match kind {
        Kind::TKind(_) => 1.,
        Kind::FKind(_) => 2.,
    };
}

fn create_layer_sprites(
    commands: &mut Commands,
    real_coordinates: (f32, f32),
    handle_map: &TerrainHandleMap,
    kind: &Kind,
    animation_indices: &AnimationIndices,
    tileset_index: usize,
) {
    let handle = handle_map.get(kind).unwrap();

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: handle.clone(),
            sprite: TextureAtlasSprite::new(
                animation_indices.clone().first * TILESET_HEIGHT + tileset_index,
            ),
            transform: Transform::from_xyz(
                real_coordinates.0,
                real_coordinates.1,
                get_zindex_from_kind(kind),
            ),
            ..default()
        },
        animation_indices.clone(),
        AnimationTimer(Timer::from_seconds(
            TIME_BETWEEN_FRAMES,
            TimerMode::Repeating,
        )),
    ));
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
    cam.projection.scaling_mode = ScalingMode::FixedVertical(300.);
    commands.spawn((cam, PanCam::default()));

    // Load the sprites
    //
    // Animated tileset MUST be saved as one column, N rows for the animation algorithm to work
    // properly

    let mut handle_map: TerrainHandleMap = HashMap::new();
    handle_map.insert(
        Kind::FKind(FeatureKind::Forest),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("sprites/terrain/forest.png"),
            Vec2::new(SPRITE_SIZE, SPRITE_SIZE),
            TILESET_WIDTH,
            TILESET_HEIGHT * ANIMATION_FRAME_COUNT,
            None,
            None,
        )),
    );
    handle_map.insert(
        Kind::FKind(FeatureKind::Ocean),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("sprites/terrain/ocean.png"),
            Vec2::new(SPRITE_SIZE, SPRITE_SIZE),
            TILESET_WIDTH,
            TILESET_HEIGHT * ANIMATION_FRAME_COUNT,
            None,
            None,
        )),
    );
    handle_map.insert(
        Kind::TKind(TerrainKind::Plain),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("sprites/terrain/plain.png"),
            Vec2::new(SPRITE_SIZE, SPRITE_SIZE),
            TILESET_WIDTH,
            TILESET_HEIGHT * ANIMATION_FRAME_COUNT,
            None,
            None,
        )),
    );

    // Indices in the tilesheet (TextureAtlas) that are composing the animation
    let animation_indices = AnimationIndices {
        first: 0,
        last: ANIMATION_FRAME_COUNT - 1,
    };

    // Create the layer sprites for every tile on the Map
    for item in &map {
        for layer in [Layer::Terrain, Layer::Feature] {
            if let Some(kind) = get_kind_of_tile_layer(item.1, layer) {
                create_layer_sprites(
                    &mut commands,
                    item.1.real_coordinates,
                    &handle_map,
                    kind,
                    &animation_indices,
                    get_tileset_index(&item.1, &map, &item.0, layer),
                );
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

// Dedicated AnimationLayer for layer tiles (plain, etc.) which have different
// sprite index computation than full tilesets ones that are layered on top of
// them.
#[derive(Component, Deref, DerefMut)]
struct BaseLayerAnimationTimer(Timer);

// Default AnimationLayer for sprites
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

/// Retrieve the next tick sprite index.
///
/// Since every tileset is animated, we have to match each tile to its future
/// counterpart in the next tileset. This way each tile will «cycle» through its
/// animated frames.
fn get_next_sprite_index(
    current_index: usize,
    indices: &AnimationIndices,
    tileset_width: usize,
    tileset_height: usize,
) -> usize {
    // We have to decompose the current sprite position into two parts:
    // - The current animation frame tileset
    let current_animation_index = current_index / (tileset_width * tileset_height);
    // - The current current_index INSIDE the current animation tileset
    let current_sprite = current_index % (tileset_width * tileset_height);
    // Now we can determine what is the next animation frame tileset
    let next_animation_index = if current_animation_index == indices.last {
        indices.first
    } else {
        current_animation_index + 1
    };
    // and recompute the proper sprite position inside this animation frame
    return next_animation_index * (tileset_width * tileset_height) + current_sprite;
}

fn animate_layer_sprite(
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
            sprite.index =
                get_next_sprite_index(sprite.index, indices, TILESET_WIDTH, TILESET_HEIGHT);
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
        .add_systems(Update, animate_layer_sprite)
        .run();
}
