use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin, PanCamSystems};

use super::constants::{SPRITE_DISPLAY_SIZE, WINDOW_SCALE_FACTOR};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin);
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, snap_camera_to_pixel.after(PanCamSystems));
    }
}

fn setup_camera(mut commands: Commands) {
    // Configure Camera that can be panned and zoomed with the mouse
    commands.spawn((Camera2d, PanCam::default()));
}

/// Snaps both the projection scale (zoom) and camera translation so that
/// tiles always occupy an exact integer number of physical pixels.
///
/// The key insight: tile seams appear whenever a tile boundary falls
/// *between* two physical pixels — the GPU must then round, and adjacent
/// tiles can round in opposite directions, leaving a gap.  If every tile
/// is guaranteed to span an exact whole number of pixels, all boundaries
/// land exactly on pixel edges and no gap can occur.
///
/// **Zoom snap** — A tile is `SPRITE_DISPLAY_SIZE` world-units wide.
/// At a given zoom its pixel width is:
///
/// ```text
///   tile_px = SPRITE_DISPLAY_SIZE * WINDOW_SCALE_FACTOR / ortho.scale
/// ```
///
/// We snap `ortho.scale` to the nearest value that makes `tile_px` an
/// integer N ≥ 1:
///
/// ```text
///   ortho.scale = SPRITE_DISPLAY_SIZE * WINDOW_SCALE_FACTOR / N
/// ```
///
/// With `SPRITE_DISPLAY_SIZE = 16` and `WINDOW_SCALE_FACTOR = 2` the base
/// product is 32, giving fine zoom steps: N = 1 → scale 32 (max dezoom),
/// N = 16 → scale 2 (initial), N = 32 → scale 1, N = 64 → scale 0.5, …
///
/// **Translation snap** — Once the zoom is locked, the pixel grid step in
/// world units is `1 / (N / SPRITE_DISPLAY_SIZE) = SPRITE_DISPLAY_SIZE / N`.
/// Equivalently `ortho.scale / WINDOW_SCALE_FACTOR`.  We round the camera
/// position to that grid.
fn snap_camera_to_pixel(mut query: Query<(&mut Transform, &mut Projection), With<Camera2d>>) {
    for (mut transform, mut projection) in query.iter_mut() {
        let Projection::Orthographic(ref mut ortho) = *projection else {
            continue;
        };

        // --- Snap the zoom level ---
        // tile_px = how many physical pixels one tile currently spans.
        let tile_px = SPRITE_DISPLAY_SIZE * WINDOW_SCALE_FACTOR / ortho.scale;
        // Round to the nearest integer ≥ 1 so tiles are always whole-pixel.
        let n = tile_px.round().max(1.0);
        ortho.scale = SPRITE_DISPLAY_SIZE * WINDOW_SCALE_FACTOR / n;

        // --- Snap the translation ---
        // One physical pixel = ortho.scale / WINDOW_SCALE_FACTOR world-units.
        let pixel_size = ortho.scale / WINDOW_SCALE_FACTOR;
        let snap = |v: f32| (v / pixel_size).round() * pixel_size;

        transform.translation.x = snap(transform.translation.x);
        transform.translation.y = snap(transform.translation.y);
    }
}
