use bevy::prelude::*;

use crate::{
    plugins::{
        MAP_HEIGHT, MAP_WIDTH, SpriteAtlas, TILE_SCALE, map::MapCoordinates, select_on_click,
    },
    state::AppState,
};

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Settler;

pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, anchor_camera_to_settler)
            .add_systems(OnEnter(AppState::ReadyToDraw), spawn_michel);
    }
}

pub fn anchor_camera_to_settler(
    settler: Single<&Transform, (With<Settler>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    camera.translation = settler.translation;
}

pub fn spawn_michel(mut commands: Commands, atlas: Res<SpriteAtlas>) {
    // Spawn our first settler !
    // His name is "Michel"

    // Spawn Michel at the Map center
    // Since the camera is linked to his position, the camera will be on the Map
    // center #BigBrainTime
    commands
        .spawn((
            atlas.sprite(&crate::plugins::SpriteType::Settler, 0, None),
            Transform {
                translation: std::convert::Into::<Vec2>::into(MapCoordinates(
                    MAP_WIDTH / 2,
                    MAP_HEIGHT / 2,
                ))
                .extend(21.),
                scale: Vec3::splat(TILE_SCALE),
                ..default()
            },
            Pickable::default(),
            Unit,
            Settler,
        ))
        .observe(select_on_click);
}
