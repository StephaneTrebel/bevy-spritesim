use std::mem::transmute;

use bevy::{
    color::palettes::css::{ROYAL_BLUE, TOMATO},
    prelude::*,
};

use crate::{
    plugins::{
        SPRITE_DISPLAY_SIZE,
        map::{MapCoordinates, MapResource},
        sprites::{SpriteAtlas, SpriteType},
        units::{Moveable, Unit},
    },
    state::AppState,
};

/// Component dedicated to the "selector" tile overlay
#[derive(Component)]
struct Selector;

/// Component dedicated to the "move selector" tile overlay
/// (that surrounds a selected unit)
#[derive(Component)]
struct MoveSelector;

/// Component added to an entity when "selected" (clicked on with the mouse)
#[derive(Component)]
struct SelectEntity;

/// Component added to an entity that can actually do stuff when selected
/// (like a unit that can be moved)
#[derive(Component)]
pub struct SelectedEntity;

#[derive(Component)]
pub struct MovingEntity;

const SELECTOR_BASE_COLOR_TINT: Color = Color::Srgba(TOMATO);
const MOVE_SELECTOR_COLOR_TINT: Color = Color::Srgba(ROYAL_BLUE);

/// Draw the Selector sprite under the Map.
/// It will be then moved to a tile (above everything) when the latter is selected.
fn draw_selector(
    mut commands: Commands,
    atlas: Res<SpriteAtlas>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    info!("Drawing selector");
    commands.spawn((
        atlas.sprite(&SpriteType::Selector, 0, Some(SELECTOR_BASE_COLOR_TINT)),
        Transform::from_xyz(0., 0., 0.0),
        Visibility::Hidden,
        Pickable::IGNORE,
        Selector,
    ));
    next_state.set(AppState::MainGame);
    info!("Done Drawing selector");
}

pub fn select_on_click(click: On<Pointer<Click>>, mut commands: Commands) {
    commands.entity(click.entity).insert(SelectEntity);
}

fn select_tile(
    mut selector: Single<(&mut Visibility, &mut Transform), With<Selector>>,
    selected: Single<(Entity, &Transform), (With<SelectEntity>, With<Unit>, Without<Selector>)>,
    mut commands: Commands,
    map_resource: Res<MapResource>,
) {
    let (entity, transform) = *selected;
    info!(
        "Entity ({},{}) selected !",
        transform.translation.x, transform.translation.y
    );
    *selector.0 = Visibility::Visible;
    selector.1.translation = Vec3 {
        x: transform.translation.x,
        y: transform.translation.y,
        z: 100.,
    };
    commands.entity(entity).remove::<SelectEntity>();
    commands.entity(entity).insert(SelectedEntity);
}

fn select_unit(
    settler: Single<(Entity, &Transform, &Unit), (With<SelectedEntity>, With<Moveable>)>,
    mut commands: Commands,
    atlas: Res<SpriteAtlas>,
    map_resource: Res<MapResource>,
    // mut next_state: ResMut<NextState<AppState>>,
) {
    let (entity, transform, unit) = *settler;
    info!("Selecting unit");

    for (x, y) in reachable_distance(
        map_resource,
        &transform.translation.into(),
        unit.speed,
    ) {
        commands.spawn((
            atlas.sprite(&SpriteType::Selector, 0, Some(MOVE_SELECTOR_COLOR_TINT)),
            Transform::from_xyz(
                transform.translation.x + (x as f32) * (SPRITE_DISPLAY_SIZE as f32),
                transform.translation.y + (y as f32) * (SPRITE_DISPLAY_SIZE as f32),
                90.,
            ),
            Pickable::IGNORE,
            MoveSelector,
        ));
    }

    commands.entity(entity).remove::<SelectedEntity>();
    commands.entity(entity).insert(MovingEntity);
    // next_state.set(AppState::UnitReadyToMove);
}

fn reachable_distance(
    map_resource: Res<MapResource>,
    &MapCoordinates(w, h): &MapCoordinates,
    distance: u16,
) -> Vec<(i32, i32)> {
    let mut tmp: Vec<(i32, i32)> = vec![];
    let speed_signed = i32::from(distance);

    for i in -speed_signed..=speed_signed {
        for j in -speed_signed..=speed_signed {
            let new_w: u16 = (w as i32 + i) as u16;
            let new_h: u16 = (h as i32 + j) as u16;
            if (i.abs() + j.abs() <= speed_signed)
                && map_resource
                    .map
                    .is_movement_allowed(MapCoordinates(new_w, new_h))
            {
                tmp.push((i, j));
            }
        }
    }
    tmp
}

fn move_unit(
    mut selected: Single<
        (Entity, &mut Transform),
        (With<MovingEntity>, With<Moveable>, Without<Selector>),
    >,
    mut selector: Single<(&mut Visibility, &mut Transform), With<Selector>>,
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    move_selectors: Query<Entity, With<MoveSelector>>,
    map_resource: Res<MapResource>,
) {
    debug!("Moving entity !");
    let entity = selected.0;
    let transform = &mut selected.1;

    if buttons.just_pressed(MouseButton::Left) {
        info!("Button pressed !");
        let window = windows.single().expect("No windows ? :(");
        if let Some(cursor_position) = window.cursor_position() {
            let (camera, camera_transform) = camera_q.single().expect("No camera ? :(");

            // Conversion écran -> monde
            if let Ok(world_position) =
                camera.viewport_to_world_2d(camera_transform, cursor_position)
            {
                info!("Click at {:?}", world_position);

                // Snap world_position to map_coordinates by converting through them
                let map_coordinates: MapCoordinates = world_position.into();
                let snapped_world_position: Vec2 = map_coordinates.into();

                if map_resource.map.is_movement_allowed(map_coordinates) {
                    info!("Moving entity to {:?}", snapped_world_position);
                    transform.translation = transform.translation.with_xy(snapped_world_position);

                    commands.entity(entity).remove::<MovingEntity>();
                }
            }
        }
        info!("Removing move_selector tiles");
        move_selectors
            .iter()
            .for_each(|move_selector| commands.entity(move_selector).despawn());
        *selector.0 = Visibility::Hidden;
    }
}

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::ReadyToDraw), draw_selector)
            .add_systems(PreUpdate, select_tile)
            .add_systems(PreUpdate, select_unit)
            .add_systems(PreUpdate, move_unit);
    }
}
