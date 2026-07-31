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
struct MoveSelector {
    spent_points: u16,
}

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
        Name::new("Selector"),
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
    entities: Query<
        (Entity, &Transform, NameOrEntity),
        (Without<ClickedEntity>, Without<Camera2d>),
    >,
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

                // @TODO Sort by Z !
                let entity = entities
                    .iter()
                    .sort_by::<(Entity, &Transform)>(|e1, e2| {
                        e2.1.translation.z.total_cmp(&e1.1.translation.z)
                    })
                    .find(|(_, t, _)| t.translation.xy() == snapped_world_position);
                if let Some((entity, _, name)) = entity {
                    info!("Entity clicked on {name}");
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
    unit_single: Single<
        (Entity, NameOrEntity, &Transform),
        (
            With<Unit>,
            With<ClickedEntity>,
            Without<SelectedEntity>,
            Without<MovingEntity>,
        ),
    >,
) {
    let mut command_entity = commands.entity(unit_single.0);
    command_entity.insert(SelectEntity);
    info!("Selecting entity {}", unit_single.1);
}

fn display_selection_selector(
    mut selector_entity: Single<(&mut Visibility, &mut Transform), With<Selector>>,
    selected_unit: Single<
        (Entity, &Transform, NameOrEntity),
        (With<SelectEntity>, With<Unit>, Without<Selector>),
    >,
    mut commands: Commands,
) {
    let mut command_entity = commands.entity(selected_unit.0);
    info!(
        "Entity ({}/{}) selected at ({},{})",
        command_entity.id(),
        selected_unit.2,
        selected_unit.1.translation.x,
        selected_unit.1.translation.y
    );
    *selector_entity.0 = Visibility::Visible;
    selector_entity.1.translation = Vec3 {
        x: selected_unit.1.translation.x,
        y: selected_unit.1.translation.y,
        z: 100.,
    };
    command_entity.remove::<SelectEntity>();
    command_entity.insert(SelectedEntity);
}

fn display_move_selectors(
    unit_single: Single<
        (Entity, &Transform, &Unit, NameOrEntity),
        (With<SelectedEntity>, With<Moveable>),
    >,
    mut commands: Commands,
    atlas: Res<SpriteAtlas>,
    map_resource: Res<MapResource>,
) {
    let entity = unit_single.0;
    let transform = unit_single.1;
    let unit = unit_single.2;

    for (x, y) in reachable_distance(
        map_resource,
        transform.translation.into(),
        unit.movement_points,
    ) {
        let transformed_x =
            transform.translation.x + f32::from(x) * (f32::from(SPRITE_DISPLAY_SIZE));
        let transformed_y =
            transform.translation.y + f32::from(y) * (f32::from(SPRITE_DISPLAY_SIZE));
        commands.spawn((
            Name::new(format!(
                "MoveSelector[(({x},{y}),({transformed_x},{transformed_y}))]"
            )),
            atlas.sprite(&SpriteType::Selector, 0, Some(MOVE_SELECTOR_COLOR_TINT)),
            Transform::from_xyz(transformed_x, transformed_y, 90.),
            Pickable::default(),
            MoveSelector {
                spent_points: (x.abs() + y.abs()) as u16,
            },
        ));
    }

    let mut command_entity = commands.entity(entity);
    info!("Selecting unit {}/{}", command_entity.id(), unit_single.3);
    command_entity.remove::<SelectedEntity>();
    command_entity.insert(MovingEntity);
}

fn reachable_distance(
    map_resource: Res<MapResource>,
    MapCoordinates(w, h): MapCoordinates,
    distance: u16,
) -> Vec<(i16, i16)> {
    let mut tmp: Vec<(i16, i16)> = vec![];
    let distance_signed: i16 = distance
        .try_into()
        .expect("Distance must be castable to i16");

    let iw: i16 = w.try_into().expect("W coordinate must be castable to i16");
    let ih: i16 = h.try_into().expect("H coordinate must be castable to i16");

    for i in -distance_signed..=distance_signed {
        for j in -distance_signed..=distance_signed {
            let new_w: u16 = (iw + i) as u16;
            let new_h: u16 = (ih + j) as u16;
            if (i.abs() + j.abs() <= distance_signed)
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
        (Entity, &mut Transform, &mut Unit),
        (Without<Selector>, With<MovingEntity>),
    >,
    mut selector: Single<
        (&mut Visibility, &mut Transform),
        (Without<ClickedEntity>, With<Selector>),
    >,
    clicked_move_selector: Single<
        (Entity, &Transform, &MoveSelector),
        (
            With<ClickedEntity>,
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
    // let transform = &mut selected_unit.1;
    // let unit = &mut selected_unit.2;

    let borrow_mut = &mut selected_unit;

    let snapped_world_position = clicked_move_selector.1.translation.xy();

    info!("Moving entity to {:?}", snapped_world_position);
    borrow_mut.1.translation = borrow_mut.1.translation.with_xy(snapped_world_position);

    info!("Spending {} movement points on entity", 2);
    borrow_mut.2.movement_points -= clicked_move_selector.2.spent_points;

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
                    select_on_click,
                    handle_click_on_entity,
                    (
                        move_unit,
                        display_selection_selector,
                        display_move_selectors,
                    ),
                ),
            );
    }
}
