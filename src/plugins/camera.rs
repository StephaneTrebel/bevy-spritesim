use bevy::{camera::ScalingMode, prelude::*};
use bevy_pancam::{PanCam, PanCamPlugin};

use super::constants::{WINDOW_PHYSICAL_HEIGHT, WINDOW_PHYSICAL_WIDTH};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default());
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // Configure Camera that can be panned and zoomed with the mouse
    commands.spawn((
        Camera2d,
        Transform::from_xyz(
            (WINDOW_PHYSICAL_WIDTH as f32) / 2.,
            (WINDOW_PHYSICAL_HEIGHT as f32) / 2.,
            0.,
        ),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 1000.,
            },
            ..OrthographicProjection::default_2d()
        }),
        PanCam::default(),
    ));
}
