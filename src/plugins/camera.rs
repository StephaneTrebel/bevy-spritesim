use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::plugins::{MAP_HEIGHT, MAP_WIDTH, MAX_SCALE, MIN_SCALE, map::MapCoordinates};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(PreUpdate, handle_mouse_scroll);
    }
}

fn setup_camera(mut commands: Commands) {
    // Configure Camera that can be panned and zoomed with the mouse
    commands.spawn((
        Camera2d,
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

fn handle_mouse_scroll(
    time: Res<Time>,
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut projection: Single<&mut Projection, With<Camera2d>>,
) {
    // @TODO: Adjust (dynamically ?)
    // @TODO: Externalize as constant ? WET principle first !
    let pixel_per_line = 50.;

    let wheel = mouse_wheel_events
        .read()
        .map(|ev| match ev.unit {
            bevy::input::mouse::MouseScrollUnit::Line => ev.y * pixel_per_line,
            bevy::input::mouse::MouseScrollUnit::Pixel => ev.y,
        })
        .sum::<f32>();

    let zoom_factor = 1.0 - (wheel * 0.1) * time.delta_secs();

    **projection = match **projection {
        Projection::Orthographic(ref orthographic_projection) => {
            let scale = orthographic_projection.scale;
            Projection::Orthographic(OrthographicProjection {
                scale: (scale * zoom_factor).clamp(MIN_SCALE, MAX_SCALE),
                ..OrthographicProjection::default_2d()
            })
        }
        _ => return,
    };
}
