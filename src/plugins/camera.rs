use bevy::{prelude::*, render::camera::ScalingMode};
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
    let mut cam = Camera2dBundle::default();
    cam.transform =
        Transform::from_xyz(WINDOW_PHYSICAL_WIDTH / 2., WINDOW_PHYSICAL_HEIGHT / 2., 0.);
    cam.projection.scaling_mode = ScalingMode::FixedVertical(1000.);
    commands.spawn((cam, PanCam::default()));
}
