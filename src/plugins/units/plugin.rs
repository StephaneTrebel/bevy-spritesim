use bevy::{
    color::palettes::css::{ROYAL_BLUE, TOMATO},
    prelude::*,
};

use crate::{
    plugins::{
        MAP_HEIGHT, MAP_WIDTH, SPRITE_DISPLAY_SIZE,
        map::{MapCoordinates, MapResource},
        selection::ClickedEntity,
        sprites::{SpriteAtlas, SpriteType},
    },
    state::AppState,
};

#[derive(Component)]
pub struct Moveable;

/// Draw the Selector sprite under the Map.
/// It will be then moved to a tile (above everything) when the latter is selected.
fn draw_selector(
    mut commands: Commands,
    atlas: Res<SpriteAtlas>,
) {
    info!("Drawing selector");
    commands.spawn((
        Name::new("Selector"),
        atlas.sprite(&SpriteType::Selector, 0, Some(SELECTOR_BASE_COLOR_TINT)),
        Transform::from_xyz(0., 0., 0.0),
        Visibility::Hidden,
        Pickable::IGNORE,
        UnitSelector,
    ));
    info!("Done Drawing selector");
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
        (Without<UnitSelector>, With<MovingEntity>),
    >,
    mut selector: Single<
        (&mut Visibility, &mut Transform),
        (Without<ClickedEntity>, With<UnitSelector>),
    >,
    clicked_move_selector: Single<
        (Entity, &Transform, &MoveSelector),
        (
            With<ClickedEntity>,
            Without<MovingEntity>,
            Without<UnitSelector>,
        ),
    >,
    move_selectors: Query<
        (Entity, &Transform),
        (
            Without<ClickedEntity>,
            With<MoveSelector>,
            Without<MovingEntity>,
            Without<UnitSelector>,
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

fn select_unit_on_click(
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

fn display_unit_selection_selector(
    mut selector_entity: Single<(&mut Visibility, &mut Transform), With<UnitSelector>>,
    selected_unit: Single<
        (Entity, &Transform, NameOrEntity),
        (With<SelectEntity>, With<Unit>, Without<UnitSelector>),
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

fn display_unit_move_selectors(
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

/// Component dedicated to the "selector" tile overlay
#[derive(Component)]
pub struct UnitSelector;

/// Component added to an entity when "selected" (clicked on with the mouse)
#[derive(Component)]
struct SelectEntity;

/// Component added to an entity that can actually do stuff when selected
/// (like a unit that can be moved)
#[derive(Component)]
pub struct SelectedEntity;

/// Component dedicated to the "move selector" tile overlay
/// (that surrounds a selected unit)
#[derive(Component)]
struct MoveSelector {
    spent_points: u16,
}

#[derive(Component)]
pub struct MovingEntity;

const SELECTOR_BASE_COLOR_TINT: Color = Color::Srgba(TOMATO);
const MOVE_SELECTOR_COLOR_TINT: Color = Color::Srgba(ROYAL_BLUE);

#[derive(Component)]
pub struct Unit {
    pub(crate) movement_speed: u16,
    pub(crate) movement_points: u16,
}

#[derive(Component)]
pub struct Settler;

pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::ReadyToDraw), spawn_michel);
        app.add_systems(OnEnter(AppState::ReadyToDraw), draw_selector);
        app.add_systems(
            PreUpdate,
            (
                select_unit_on_click,
                move_unit,
                display_unit_selection_selector,
                display_unit_move_selectors,
            ),
        );
    }
}

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

    commands.spawn((
        atlas.sprite(&SpriteType::Settler, 0, None),
        Transform {
            translation: std::convert::Into::<Vec2>::into(map_coordinates).extend(21.),
            ..default()
        },
        Pickable::default(),
        Name::new("Michel"),
        Unit {
            movement_speed: 2,
            movement_points: 2,
        },
        Settler,
        Moveable,
    ));
}
