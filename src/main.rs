use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};
use bevy::log::LogPlugin;
use bevy::winit::WinitSettings;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowLevel, WindowResolution},
};
use plugins::constants::{WINDOW_PHYSICAL_HEIGHT, WINDOW_PHYSICAL_WIDTH, WINDOW_SCALE_FACTOR};

use crate::plugins::{
    ButtonsPlugin, CameraPlugin, CustomFpsOverlayPlugin, KeyboardPlugin, MapPlugin,
    SelectionPlugin, SpritePlugin, TurnPlugin, UnitPlugin,
};
use crate::state::AppState;

mod plugins;
mod state;

/// There we go !
fn main() {
    let mut app = App::new();

    // Bevy base plugins added with "all-in-one" DefaultPlugins plugin
    app.add_plugins(
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
    )
    .insert_resource(
        // Update as fast as possible (no downgrade when losing focus)
        WinitSettings::continuous(),
    );

    // Bevy third-party plugins
    app.add_plugins(DebugPickingPlugin)
        // Switch to show Debug overlay
        .insert_resource(DebugPickingMode::Disabled);

    // Our game plugins
    app.add_plugins((
        ButtonsPlugin,
        CameraPlugin,
        CustomFpsOverlayPlugin,
        KeyboardPlugin,
        MapPlugin,
        SelectionPlugin,
        SpritePlugin,
        UnitPlugin,
        TurnPlugin,
    ));

    // Game state (Menu, Map, etc.)
    app.init_state::<AppState>();

    // Let's-a go !
    app.run();
}
