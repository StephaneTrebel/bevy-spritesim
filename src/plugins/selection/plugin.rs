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

/// Component added to an entity that has been clicked on
/// (like a unit that can be moved, or a target destination for a unit movement)
#[derive(Component)]
pub struct ClickedEntity;

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

fn handle_click_on_entity(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    currently_clicked_entity: Option<Single<Entity, With<ClickedEntity>>>,
    entities: Query<(Entity, &Transform), Without<ClickedEntity>>,
) {
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
                info!("snapped_world_position {snapped_world_position}");

                let entity = entities
                    .iter()
                    .find(|(_, t)| t.translation.xy() == snapped_world_position);
                if let Some((entity, _)) = entity {
                    info!("Entity clicked on {entity:?}");
                    if let Some(entity) = currently_clicked_entity {
                        commands.entity(entity.entity()).remove::<ClickedEntity>();
                    }
                    commands.entity(entity).insert(ClickedEntity);
                } else {
                    info!("No entity there.");
                }
            }
        }
    }
}

fn select_on_click(
    mut commands: Commands,
    unit: Single<
        (Entity, &Transform),
        (
            With<Unit>,
            With<ClickedEntity>,
            Without<SelectedEntity>,
            Without<MovingEntity>,
        ),
    >,
) {
    debug!("Selecting entity !");
    commands.entity(unit.0).insert(SelectEntity);
}

fn display_selection_selector(
    mut selector_entity: Single<(&mut Visibility, &mut Transform), With<Selector>>,
    selected_unit: Single<
        (Entity, &Transform),
        (With<SelectEntity>, With<Unit>, Without<Selector>),
    >,
    mut commands: Commands,
) {
    let (entity, transform) = *selected_unit;
    info!(
        "Entity ({},{}) selected !",
        transform.translation.x, transform.translation.y
    );
    *selector_entity.0 = Visibility::Visible;
    selector_entity.1.translation = Vec3 {
        x: transform.translation.x,
        y: transform.translation.y,
        z: 100.,
    };
    commands.entity(entity).remove::<SelectEntity>();
    commands.entity(entity).insert(SelectedEntity);
}

fn display_move_selectors(
    settler: Single<(Entity, &Transform, &Unit), (With<SelectedEntity>, With<Moveable>)>,
    mut commands: Commands,
    atlas: Res<SpriteAtlas>,
    map_resource: Res<MapResource>,
    // mut next_state: ResMut<NextState<AppState>>,
) {
    let (entity, transform, unit) = *settler;
    info!("Selecting unit");

    for (x, y) in reachable_distance(map_resource, transform.translation.into(), unit.speed) {
        commands.spawn((
            atlas.sprite(&SpriteType::Selector, 0, Some(MOVE_SELECTOR_COLOR_TINT)),
            Transform::from_xyz(
                transform.translation.x + (x as f32) * (f32::from(SPRITE_DISPLAY_SIZE)),
                transform.translation.y + (y as f32) * (f32::from(SPRITE_DISPLAY_SIZE)),
                90.,
            ),
            Pickable::default(),
            MoveSelector,
        ));
    }

    commands.entity(entity).remove::<SelectedEntity>();
    commands.entity(entity).insert(MovingEntity);
}

fn reachable_distance(
    map_resource: Res<MapResource>,
    MapCoordinates(w, h): MapCoordinates,
    distance: u16,
) -> Vec<(i32, i32)> {
    let mut tmp: Vec<(i32, i32)> = vec![];
    let speed_signed = i32::from(distance);

    for i in -speed_signed..=speed_signed {
        for j in -speed_signed..=speed_signed {
            let new_w: u16 = (i32::from(w) + i) as u16;
            let new_h: u16 = (i32::from(h) + j) as u16;
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
    mut commands: Commands,
    mut selected_unit: Single<
        (Entity, &mut Transform),
        (
            With<Unit>,
            Without<Selector>,
            With<MovingEntity>,
        ),
    >,
    mut selector: Single<
        (&mut Visibility, &mut Transform),
        (Without<ClickedEntity>, With<Selector>),
    >,
    clicked_move_selector: Single<
        (Entity, &Transform),
        (
            With<ClickedEntity>,
            With<MoveSelector>,
            Without<MovingEntity>,
            Without<Selector>,
        ),
    >,
    move_selectors: Query<
        (Entity, &Transform),
        (
            Without<ClickedEntity>,
            With<MoveSelector>,
            Without<MovingEntity>,
            Without<Selector>,
        ),
    >,
) {
    debug!("Moving entity !");
    let entity = selected_unit.0;
    let transform = &mut selected_unit.1;

    let snapped_world_position = clicked_move_selector.1.translation.xy();

    info!("Moving entity to {:?}", snapped_world_position);
    transform.translation = transform.translation.with_xy(snapped_world_position);

    commands.entity(entity).remove::<MovingEntity>();
    info!("Removing move_selector tiles");
    commands.entity(clicked_move_selector.0).despawn();
    move_selectors
        .iter()
        .for_each(|(entity, _)| commands.entity(entity).despawn());
    *selector.0 = Visibility::Hidden;
}

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::ReadyToDraw), draw_selector)
            .add_systems(
                PreUpdate,
                (
                    handle_click_on_entity,
                    (
                        move_unit,
                        display_selection_selector,
                        display_move_selectors,
                    ),
                    select_on_click,
                ),
            );
    }
}
