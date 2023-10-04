use bevy::{math::vec2, prelude::*, render::camera::ScalingMode, window::*};
use bevy_pancam::{PanCam, PanCamPlugin};
use noisy_bevy::{fbm_simplex_2d, simplex_noise_2d};
// use rand::{rngs::StdRng, Rng, SeedableRng};

const WINDOW_PHYSICAL_WIDTH: f32 = 1280.; // In pixels
const WINDOW_PHYSICAL_HEIGHT: f32 = 1280.; // In pixels
const WINDOW_SCALE_FACTOR: f64 = 2.0; // How much tiles are streched out in the beginning
const SPRITE_SIZE: f32 = 32.;
const MAP_WIDTH: usize = 100;
const MAP_HEIGHT: usize = 100;

enum Kind {
    Plain,
    Ocean,
    // Forest,
}

struct Tile {
    kind: Kind,
    transform: Transform,
}

fn build_map_v2(width: i32, height: i32) -> Vec<Tile> {
    let mut map = Vec::new();

    // Init with Ocean tiles
    for w in 0..width {
        for h in 0..height {
            map.push(Tile {
                kind: Kind::Ocean,
                transform: Transform::from_xyz(
                    (w as f32) * SPRITE_SIZE
                        - (WINDOW_PHYSICAL_WIDTH / (2. * WINDOW_SCALE_FACTOR as f32)
                            - SPRITE_SIZE / 2.),
                    (h as f32) * SPRITE_SIZE
                        - (WINDOW_PHYSICAL_HEIGHT / (2. * WINDOW_SCALE_FACTOR as f32)
                            - SPRITE_SIZE / 2.),
                    0.,
                ),
            });
        }
    }

    // Setup constants for noisy_bevy
    const FREQUENCY_SCALE: f32 = 0.2;
    const AMPLITUDE_SCALE: f32 = 8.0;
    const RADIUS: f32 = 30.;

    // Generate a patch of Plain in the middle
    let grid_half_size = RADIUS as i32 + 1;
    for w in -grid_half_size..=grid_half_size {
        for h in -grid_half_size..=grid_half_size {
            let p = vec2(w as f32, h as f32);

            // Compute noise offset (so the "blob" shape the patch will have)
            let offset = simplex_noise_2d(p * FREQUENCY_SCALE) * AMPLITUDE_SCALE;

            // Height will serve, with a cutoff, as sizing the resulting patch
            let height = RADIUS + offset - ((w * w + h * h) as f32).sqrt();
            let min_height = -1.;
            dbg!(w, h, height, offset);

            if height > min_height {
                map.push(Tile {
                    kind: Kind::Plain,
                    transform: Transform::from_translation(Vec3::new(
                        (w as f32) * SPRITE_SIZE + WINDOW_PHYSICAL_WIDTH,
                        (h as f32) * SPRITE_SIZE + WINDOW_PHYSICAL_HEIGHT,
                        1.,
                    )),
                });
            }
        }
    }

    // Generate a patch of Forest

    return map;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // PRGN initialization
    // let seed = StdRng::from_entropy().gen_range(0..MAX);
    // println!("seed={:?}", dbg!(&seed));
    // let mut rng = StdRng::seed_from_u64(seed);

    // Map generation
    let map = build_map_v2(MAP_WIDTH as i32, MAP_HEIGHT as i32);

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
    for (_, item) in map.into_iter().enumerate() {
        commands.spawn(SpriteBundle {
            texture: match item.kind {
                // Kind::Forest => forest_sprite_handle.clone(),
                Kind::Ocean => ocean_sprite_handle.clone(),
                Kind::Plain => plain_sprite_handle.clone(),
            },
            transform: item.transform,
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
