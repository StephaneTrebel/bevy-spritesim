use std::u32::MAX as MAX_u32;
use std::u64::MAX as MAX_u64;

use bevy::{math::vec2, prelude::*, render::camera::ScalingMode, utils::HashMap, window::*};
use bevy_pancam::{PanCam, PanCamPlugin};
use noisy_bevy::simplex_noise_2d_seeded;
use rand::{rngs::StdRng, Rng, SeedableRng};

const WINDOW_PHYSICAL_WIDTH: f32 = 1280.; // In pixels
const WINDOW_PHYSICAL_HEIGHT: f32 = 1280.; // In pixels
const WINDOW_SCALE_FACTOR: f64 = 2.0; // How much tiles are streched out in the beginning
const SPRITE_SIZE: f32 = 32.;
const MAP_WIDTH: i32 = 100;
const MAP_HEIGHT: i32 = 100;

// Setup constants for noisy_bevy
const BASE_FREQUENCY_SCALE: f32 = 0.05;
const BASE_AMPLITUDE_SCALE: f32 = 4.0;

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
            if *kind == Kind::Forest {
                dbg!(height);
            }
            let min_height = -1.;

            let key = (coordinates.0 + w, coordinates.1 + h);
            // Only replace tile when necessary (for instance, Forest tiles can only be placed on Plains)
            let replace = !(*kind == Kind::Forest && map.get(&key).unwrap().kind != Kind::Plain);
            if replace
                && height > min_height
                && key.0 > 0
                && key.1 > 0
                && key.0 < MAP_WIDTH
                && key.1 < MAP_HEIGHT
            {
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

fn generate_multiple_patches(
    pseudo_rng_instance: &mut StdRng,
    mut map: &mut Map,
    kind: Kind,
    count: i32,
    min_radius: i32,
    max_radius: i32,
    delta_frequency_scale: f32,
    delta_amplitude_scale: f32,
) {
    let max_offset = 5;
    let mut points: Vec<(i32, i32)> = Vec::new();
    for w in 1..count {
        for h in 1..count {
            points.push((
                pseudo_rng_instance.gen_range(-max_offset..=max_offset)
                    + MAP_WIDTH as i32 * w / count,
                pseudo_rng_instance.gen_range(-max_offset..=max_offset)
                    + MAP_HEIGHT as i32 * h / count,
            ));
        }
    }
    for coordinates in points {
        generate_patch(
            pseudo_rng_instance.gen_range(0..MAX_u32) as f32,
            &mut map,
            &kind,
            coordinates,
            pseudo_rng_instance.gen_range(min_radius..max_radius) as f32,
            pseudo_rng_instance.gen_range(
                BASE_FREQUENCY_SCALE - delta_frequency_scale
                    ..BASE_FREQUENCY_SCALE + delta_frequency_scale,
            ),
            pseudo_rng_instance.gen_range(
                BASE_AMPLITUDE_SCALE - delta_amplitude_scale
                    ..BASE_AMPLITUDE_SCALE + delta_amplitude_scale,
            ),
        );
    }
}

fn build_map(mut pseudo_rng_instance: &mut StdRng) -> Map {
    let map_seed = pseudo_rng_instance.gen_range(0..MAX_u64);
    dbg!(map_seed);
    let mut map: Map = HashMap::new();

    // Init with Ocean tiles
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
    // (but with an irregular shape)
    generate_multiple_patches(
        &mut pseudo_rng_instance,
        &mut map,
        Kind::Plain,
        6,
        2,
        20,
        0.01,
        0.4,
    );

    // Generate randow patches of Forests
    generate_multiple_patches(
        &mut pseudo_rng_instance,
        &mut map,
        Kind::Forest,
        6,
        1,
        5,
        0.05,
        0.8,
    );

    return map;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    let forest_sprite_handle = asset_server.load("sprites/terrain/forest.png");
    let ocean_sprite_handle = asset_server.load("sprites/terrain/ocean.png");
    let plain_sprite_handle = asset_server.load("sprites/terrain/plain.png");

    // Display the sprites
    for item in map {
        commands.spawn(SpriteBundle {
            texture: match item.1.kind {
                Kind::Forest => forest_sprite_handle.clone(),
                Kind::Ocean => ocean_sprite_handle.clone(),
                Kind::Plain => plain_sprite_handle.clone(),
            },
            transform: item.1.transform,
            ..default()
        });
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
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
            }),
            GamePlugin,
            PanCamPlugin::default(),
        ))
        .run();
}
