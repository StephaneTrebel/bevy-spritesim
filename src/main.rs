use bevy::{prelude::*, window::*};
use rand::{rngs::StdRng, Rng, SeedableRng};

const WINDOW_PHYSICAL_WIDTH: f32 = 640.;
const WINDOW_PHYSICAL_HEIGHT: f32 = 512.;
const WINDOW_SCALE_FACTOR: f64 = 2.0;

enum Color {
    Plain,
    Ocean,
    Forest,
}

struct Tile {
    coordinates: Vec3,
    color: Color,
}

fn build_map(width: i32, height: i32, rng: &mut StdRng) -> Vec<Tile> {
    let mut map = Vec::new();
    let offset_width = WINDOW_PHYSICAL_WIDTH / (2. * WINDOW_SCALE_FACTOR as f32);
    let offset_height = WINDOW_PHYSICAL_HEIGHT / (2. * WINDOW_SCALE_FACTOR as f32);
    for w in 0..width {
        for h in 0..height {
            let test = rng.gen_range(0..3);
            map.push(Tile {
                coordinates: Vec3::from((
                    (w as f32) * 32. - offset_width,
                    (h as f32) * 32. - offset_height,
                    0.,
                )),
                color: match test {
                    1 => Color::Plain,
                    2 => Color::Ocean,
                    _ => Color::Forest,
                },
            });
        }
    }
    return map;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = StdRng::seed_from_u64(19878367467713);
    let map = build_map(10, 10, &mut rng);
    commands.spawn(Camera2dBundle::default());

    for item in map {
        commands.spawn(SpriteBundle {
            texture: match item.color {
                Color::Plain => asset_server.load("sprites/terrain/plain.png"),
                Color::Ocean => asset_server.load("sprites/terrain/ocean.png"),
                Color::Forest => asset_server.load("sprites/terrain/forest.png"),
            },
            transform: Transform::from_xyz(
                item.coordinates.x as f32,
                item.coordinates.y as f32,
                item.coordinates.z as f32,
            ),
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
        ))
        .run();
}
