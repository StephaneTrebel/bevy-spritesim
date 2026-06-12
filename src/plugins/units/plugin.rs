use bevy::prelude::*;
use bevy_pancam::PanCam;

use crate::{
    plugins::{
        MAP_HEIGHT, MAP_WIDTH, MovingEntity, SelectedEntity, SpriteAtlas,
        map::{MapCoordinates, MapResource},
        select_on_click,
    },
    state::AppState,
};

#[derive(Component)]
pub struct Moveable;

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Settler;

pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::ReadyToDraw), spawn_michel);
    }
}

#[derive(Component)]
struct AnchorCamera;

/// Spawn our first settler !
/// His name is "Michel"
pub fn spawn_michel(
    mut commands: Commands,
    atlas: Res<SpriteAtlas>,
    map_resource: Res<MapResource>,
) {
    // Find a possible spawn point for Michel
    let mut map_coordinates = MapCoordinates(MAP_WIDTH / 2, MAP_HEIGHT / 2);
    while !map_resource.map.is_movement_allowed(map_coordinates) {
        map_coordinates = MapCoordinates(map_coordinates.0 + 1, map_coordinates.1 + 1);
    }

    commands
        .spawn((
            atlas.sprite(&crate::plugins::SpriteType::Settler, 0, None),
            Transform {
                translation: std::convert::Into::<Vec2>::into(map_coordinates).extend(21.),
                ..default()
            },
            Pickable::default(),
            Unit,
            Settler,
            Moveable,
        ))
        .observe(select_on_click);
}
