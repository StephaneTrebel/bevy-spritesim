use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin);
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // Configure Camera that can be panned and zoomed with the mouse
    commands.spawn((Camera2d, PanCam::default()));
}
