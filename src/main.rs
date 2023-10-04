use bevy::{prelude::*, window::*};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/terrain/red.png"),
        transform: Transform::from_xyz(100., 0., 0.),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/terrain/green.png"),
        transform: Transform::from_xyz(50., 50., 0.),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/terrain/blue.png"),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
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
