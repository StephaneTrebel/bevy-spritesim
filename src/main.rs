use bevy::{prelude::*, window::*};
use rand::{rngs::StdRng, Rng, SeedableRng};

enum Color {
    Red,
    Green,
    Blue,
}

struct Tile {
    coordinates: Vec3,
    color: Color,
}

fn build_map(width: i32, height: i32, rng: &mut StdRng) -> Vec<Tile> {
    let mut map = Vec::new();
    for w in 0..width {
        for h in 0..height {
            let test = rng.gen_range(0..3);
            map.push(Tile {
                coordinates: Vec3::from(((w as f32) * 32., (h as f32) * 32., 0.)),
                color: match test {
                    1 => Color::Red,
                    2 => Color::Blue,
                    _ => Color::Green,
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
                Color::Red => asset_server.load("sprites/terrain/red.png"),
                Color::Blue => asset_server.load("sprites/terrain/blue.png"),
                Color::Green => asset_server.load("sprites/terrain/green.png"),
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
                    resolution: (640., 480.).into(),
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
