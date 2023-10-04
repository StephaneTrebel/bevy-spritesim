use bevy::{prelude::*, window::*};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

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

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, setup);
        // .add_systems(Update, greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
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
            HelloPlugin,
        ))
        .run();
}
