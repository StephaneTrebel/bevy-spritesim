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
            CustomFpsOverlayPlugin
        ))
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::ReadyToDraw), draw_map)
        .run();
}
