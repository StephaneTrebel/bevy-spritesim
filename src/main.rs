use bevy::log::LogPlugin;
use bevy::winit::WinitSettings;
use bevy::{prelude::*, window::*};
use plugins::camera::CameraPlugin;
use plugins::constants::{WINDOW_PHYSICAL_HEIGHT, WINDOW_PHYSICAL_WIDTH, WINDOW_SCALE_FACTOR};

use crate::plugins::CustomFpsOverlayPlugin;
use crate::plugins::map::{MapPlugin, draw_map};
use crate::plugins::sprites::SpritePlugin;
use crate::state::AppState;

mod plugins;
mod state;

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
                        present_mode: PresentMode::AutoNoVsync,
                        window_level: WindowLevel::AlwaysOnTop,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin { ..default() }),
            SpritePlugin,
            MapPlugin,
            CameraPlugin,
            CustomFpsOverlayPlugin,
            bevy_framepace::FramepacePlugin,
        ))
        .insert_resource(
            // Update as fast as possible (no downgrade when losing focus)
            WinitSettings::continuous(),
        )
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::ReadyToDraw), draw_map)
        .run();
}
