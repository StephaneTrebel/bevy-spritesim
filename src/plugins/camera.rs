use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

use crate::plugins::{MAP_HEIGHT, MAP_WIDTH, map::MapCoordinates};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin);
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // Configure Camera that can be panned and zoomed with the mouse
    commands.spawn((
        Camera2d,
        PanCam::default(),
        Transform {
            translation: std::convert::Into::<Vec2>::into(MapCoordinates(
                MAP_WIDTH / 2,
                MAP_HEIGHT / 2,
            ))
            .extend(999.),
            ..default()
        },
    ));
}
