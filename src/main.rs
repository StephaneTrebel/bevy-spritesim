use std::u64::MAX;

use bevy::{math::vec2, prelude::*, render::camera::ScalingMode, utils::HashMap, window::*};
use bevy_pancam::{PanCam, PanCamPlugin};
use noisy_bevy::simplex_noise_2d;
use rand::{rngs::StdRng, Rng, SeedableRng};

const WINDOW_PHYSICAL_WIDTH: f32 = 1280.; // In pixels
const WINDOW_PHYSICAL_HEIGHT: f32 = 1280.; // In pixels
const WINDOW_SCALE_FACTOR: f64 = 2.0; // How much tiles are streched out in the beginning
const SPRITE_SIZE: f32 = 32.;
const MAP_WIDTH: usize = 100;
const MAP_HEIGHT: usize = 100;

// Setup constants for noisy_bevy
const FREQUENCY_SCALE: f32 = 0.2;
const AMPLITUDE_SCALE: f32 = 8.0;

#[derive(Clone, Debug)]
enum Kind {
    Plain,
    Ocean,
    // Forest,
}

#[derive(Debug)]
struct Tile {
    kind: Kind,
    transform: Transform,
}

type Map = HashMap<(i32, i32), Tile>;

fn generate_patch(map: &mut Map, kind: Kind, coordinates: (i32, i32), radius: f32) {
    let grid_half_size = radius as i32 + 1;
    for w in -grid_half_size..=grid_half_size {
        for h in -grid_half_size..=grid_half_size {
            let p = vec2(w as f32, h as f32);

            // Compute noise offset (That will contribute to the "blob" shape
            // the patch will have)
            let offset = simplex_noise_2d(p * FREQUENCY_SCALE) * AMPLITUDE_SCALE;

            // Height will serve, with a threshold cutoff, as sizing the resulting patch
            let height = radius + offset - ((w * w + h * h) as f32).sqrt();
            let min_height = -1.;

            if height > min_height {
                map.insert(
                    (coordinates.0 + w, coordinates.1 + h),
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

fn build_map(width: i32, height: i32, rng: &mut StdRng) -> Map {
    let mut map: Map = HashMap::new();

    // Init with Ocean tiles
    for w in 0..width {
        for h in 0..height {
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

    // Generate patches of Plain to serve as a main continent
    // (but with an irregular shape)
    let max_offset = 3; // Maximum offset from the original starting spot
    for coordinates in [
        (
            rng.gen_range(-max_offset..=max_offset) + width as i32 / 3,
            rng.gen_range(-max_offset..=max_offset) + height as i32 / 3,
        ),
        (
            rng.gen_range(-max_offset..=max_offset) + width as i32 / 3,
            rng.gen_range(-max_offset..=max_offset) + height as i32 * 2 / 3,
        ),
        (
            rng.gen_range(-max_offset..=max_offset) + width as i32 * 2 / 3,
            rng.gen_range(-max_offset..=max_offset) + height as i32 / 3,
        ),
        (
            rng.gen_range(-max_offset..=max_offset) + width as i32 * 2 / 3,
            rng.gen_range(-max_offset..=max_offset) + height as i32 * 2 / 3,
        ),
    ] {
        generate_patch(
            &mut map,
            Kind::Plain,
            coordinates,
            rng.gen_range(15..20) as f32,
        );
    }

    // Generate a patch of Forest

    return map;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // PRGN initialization
    let seed = StdRng::from_entropy().gen_range(0..MAX);
    let mut rng = StdRng::seed_from_u64(seed);

    // Map generation
    let map = build_map(MAP_WIDTH as i32, MAP_HEIGHT as i32, &mut rng);

    // Configure Camera that can be panned and zoomed with the mouse
    let mut cam = Camera2dBundle::default();
    cam.transform =
        Transform::from_xyz(WINDOW_PHYSICAL_WIDTH / 2., WINDOW_PHYSICAL_HEIGHT / 2., 0.);
    cam.projection.scaling_mode = ScalingMode::FixedVertical(5000.);
    commands.spawn((cam, PanCam::default()));

    // Load the sprites
    // let forest_sprite_handle = asset_server.load("sprites/terrain/forest.png");
    let ocean_sprite_handle = asset_server.load("sprites/terrain/ocean.png");
    let plain_sprite_handle = asset_server.load("sprites/terrain/plain.png");

    // Display the sprites
    for item in map {
        commands.spawn(SpriteBundle {
            texture: match item.1.kind {
                // Kind::Forest => forest_sprite_handle.clone(),
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
                    focused: false,
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
