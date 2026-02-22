use bevy::{prelude::*, window::*};
use plugins::camera::CameraPlugin;
use plugins::constants::{WINDOW_PHYSICAL_HEIGHT, WINDOW_PHYSICAL_WIDTH, WINDOW_SCALE_FACTOR};

use crate::plugins::map::{MapPlugin, MapResource};
use crate::plugins::sprites::SpritePlugin;
use crate::plugins::{SPRITE_SIZE, SpriteAtlas};
use crate::state::AppState;

mod plugins;
mod state;

fn draw_map(mut commands: Commands, atlas: Res<SpriteAtlas>, map: Res<MapResource>) {
    for (&map_cooordinates, tile) in map.map.iter() {
        for (_layer, &(kind, variant)) in tile.layers.iter() {
            commands.spawn((
                atlas.sprite(&kind.get_sprite_type(), variant),
                Transform {
                    translation: Vec3::new(
                        f32::from(map_cooordinates.0 as u16) * SPRITE_SIZE,
                        f32::from(map_cooordinates.1 as u16) * SPRITE_SIZE,
                        0.,
                    ),
                    scale: Vec3::splat(1.),
                    ..default()
                },
            ));
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
            SpritePlugin,
            MapPlugin,
            CameraPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::ReadyToDraw), draw_map)
        .run();
}
