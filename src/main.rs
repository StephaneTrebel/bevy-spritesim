use bevy::{math::vec2, prelude::*, render::camera::ScalingMode, window::*};
use bevy_pancam::{PanCam, PanCamPlugin};
use noisy_bevy::fbm_simplex_2d;
// use rand::{rngs::StdRng, Rng, SeedableRng};

const WINDOW_PHYSICAL_WIDTH: f32 = 1280.;
const WINDOW_PHYSICAL_HEIGHT: f32 = 1280.;
const WINDOW_SCALE_FACTOR: f64 = 2.0;
const OFFSET_WIDTH: f32 =
    WINDOW_PHYSICAL_WIDTH / (2. * WINDOW_SCALE_FACTOR as f32) - SPRITE_SIZE / 2.;
const OFFSET_HEIGHT: f32 =
    WINDOW_PHYSICAL_HEIGHT / (2. * WINDOW_SCALE_FACTOR as f32) - SPRITE_SIZE / 2.;
const SPRITE_SIZE: f32 = 32.;

enum Kind {
    Plain,
    Ocean,
    // Forest,
}

struct Tile {
    kind: Kind,
}

fn build_map_v2(width: i32, height: i32) -> Vec<Tile> {
    let mut map = Vec::new();

    // Init with Ocean tiles
    for _ in 0..width * height {
        map.push(Tile { kind: Kind::Ocean });
    }

    // Setup constants for noisy_bevy
    const FREQUENCY_SCALE: f32 = 0.05;
    const AMPLITUDE_SCALE: f32 = 4.0;
    const RADIUS: f32 = 5.;
    const OCTAVES: usize = 3;
    const LACUNARITY: f32 = 2.;
    const GAIN: f32 = 0.5;

    // Generate a patch of Plain in the middle
    let grid = 2 * RADIUS as i32;
    for x in 0..=grid {
        print!("x: {} ", x);
        for y in 0..=grid {
            print!("y: {} ", y);
            let p = vec2(x as f32, y as f32);

            // Compute noise offset (so the "blob" shape the patch will have)
            let offset =
                fbm_simplex_2d(p * FREQUENCY_SCALE, OCTAVES, LACUNARITY, GAIN) * AMPLITUDE_SCALE;

            // Height will serve, with a cutoff, as sizing the resulting patch
            let height = ((x * x + y * y) as f32).sqrt() - (RADIUS + offset);
            println!("height: {}", height);
            if height > 5. {
                map[(x * width + y) as usize].kind = Kind::Plain;
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
    const MAP_WIDTH: usize = 20;
    const MAP_HEIGHT: usize = 20;
    let map = build_map_v2(MAP_WIDTH as i32, MAP_HEIGHT as i32);

    // Configure Camera that can be panned and zoomed with the mouse
    let mut cam = Camera2dBundle::default();
    cam.projection.scaling_mode = ScalingMode::FixedVertical(500.);
    commands.spawn((cam, PanCam::default()));

    // Load the sprites
    let ocean_sprite_handle = asset_server.load("sprites/terrain/ocean.png");
    let plain_sprite_handle = asset_server.load("sprites/terrain/plain.png");

    // Display the sprites
    for (index, item) in map.into_iter().enumerate() {
        commands.spawn(SpriteBundle {
            texture: match item.kind {
                // Kind::Forest => asset_server.load("sprites/terrain/forest.png"),
                Kind::Ocean => ocean_sprite_handle.clone(),
                Kind::Plain => plain_sprite_handle.clone(),
            },
            transform: {
                let w = index / MAP_WIDTH;
                let h = index - (w * MAP_WIDTH);
                println!("index = {}, w = {}, h = {}", index, w, h);
                Transform::from_xyz(
                    (w as f32) * SPRITE_SIZE - OFFSET_WIDTH,
                    (h as f32) * SPRITE_SIZE - OFFSET_HEIGHT,
                    0.,
                )
            },
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
