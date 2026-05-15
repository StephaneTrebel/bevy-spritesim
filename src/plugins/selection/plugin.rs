use bevy::{
    color::palettes::css::{ROYAL_BLUE, TOMATO},
    prelude::*,
};

use crate::{
    plugins::{
        H_OFFSET, SPRITE_DISPLAY_SIZE, SpriteAtlas, W_OFFSET,
        map::{RealCoordinates, Settler},
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
struct SelectedEntity;

#[derive(Component)]
struct MovingEntity;

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
        atlas.sprite(
            &crate::plugins::SpriteType::Selector,
            0,
            Some(SELECTOR_BASE_COLOR_TINT),
        ),
        Transform::from_xyz(0., 0., 0.0),
        Visibility::Hidden,
        Pickable::IGNORE,
        Selector,
    ));
    next_state.set(AppState::MainGame);
}

pub fn select_on_click(click: On<Pointer<Click>>, mut commands: Commands) {
    commands.entity(click.entity).insert(SelectEntity);
}

fn select_tile(
    selected: Single<(Entity, &RealCoordinates), With<SelectEntity>>,
    mut selector: Single<(&mut Visibility, &mut Transform), With<Selector>>,
    mut commands: Commands,
) {
    info!("Entity ({},{}) selected !", selected.1.x, selected.1.y);
    *selector.0 = Visibility::Visible;
    selector.1.translation = Vec3 {
        x: selected.1.x,
        y: selected.1.y,
        z: 100.,
    };
    commands.entity(selected.0).remove::<SelectEntity>();
    commands.entity(selected.0).insert(SelectedEntity);
}

fn select_unit(
    settler: Single<(Entity, &RealCoordinates, &Settler), With<SelectedEntity>>,
    mut commands: Commands,
    atlas: Res<SpriteAtlas>,

    mut next_state: ResMut<NextState<AppState>>,
) {
    info!("Selecting unit");
    let real_coordinates = settler.1;

    for (x, y) in [
        (-1., -1.),
        (-1., 0.),
        (-1., 1.),
        (1., 1.),
        (1., 0.),
        (1., -1.),
        (0., -1.),
        (0., 1.),
    ] {
        commands.spawn((
            atlas.sprite(
                &crate::plugins::SpriteType::Selector,
                0,
                Some(MOVE_SELECTOR_COLOR_TINT),
            ),
            Transform::from_xyz(
                real_coordinates.x + x * SPRITE_DISPLAY_SIZE,
                real_coordinates.y + y * SPRITE_DISPLAY_SIZE,
                90.,
            ),
            Pickable::IGNORE,
            MoveSelector,
        ));
    }

    commands.entity(settler.0).remove::<SelectedEntity>();
    commands.entity(settler.0).insert(MovingEntity);
    // next_state.set(AppState::UnitReadyToMove);
}

fn move_unit(
    mut settler: Single<(Entity, &mut RealCoordinates, &Settler), With<MovingEntity>>,
    mut commands: Commands,
    // mut next_state: ResMut<NextState<AppState>>,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    move_selectors: Query<Entity, With<MoveSelector>>,
    mut selector: Single<(&mut Visibility, &mut Transform), With<Selector>>,
) {
    info!("Moving entity !");
    // let real_coordinates = settler.1;

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

                // TODO: Uncomment me !
                // let new_x = (world_position.x + W_OFFSET) / SPRITE_DISPLAY_SIZE;
                // let new_y = (world_position.y + H_OFFSET) / SPRITE_DISPLAY_SIZE;
                // info!("Moving entity to {:?}", (new_x, new_y));

                // Méthode cracrapourlinstant: on met directement à jour la
                // position de l'unité dans la fenêtre :D
                settler.1.x = world_position.x;
                settler.1.y = world_position.y;

                commands.entity(settler.0).remove::<MovingEntity>();
            }

            // next_state.set(AppState::UnitReadyToMove);
        }
        info!("Removing move_selector tiles");
        move_selectors
            .iter()
            .for_each(|selector| commands.entity(selector).despawn());
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
