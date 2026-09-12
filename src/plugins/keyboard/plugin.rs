use bevy::prelude::*;

use crate::{
    plugins::{
        main_menu::IsMainMenuShown,
        sprites::{SpriteAtlas, SpriteType},
        units::Unit,
    },
    state::AppState,
};

#[derive(Component)]
pub struct Village;

pub struct KeyboardPlugin;
impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, handle_input.run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(AppState::ReadyToDraw), draw_village);
    }
}

pub fn draw_village(mut commands: Commands, atlas: Res<SpriteAtlas>) {
    info!("Drawing village sprite");
    commands.spawn((
        Name::new("Village"),
        atlas.sprite(&SpriteType::Village, 0, None),
        Transform::from_xyz(0., 0., 0.0),
        Visibility::Hidden,
        Pickable::IGNORE,
        Village,
    ));
}

fn handle_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    unit: Single<(Entity, &Transform), (With<Unit>, Without<Village>)>,
    mut village: Single<(&mut Visibility, &mut Transform), With<Village>>,
    main_menu_current_state: Res<State<IsMainMenuShown>>,
    mut main_menu_next_state: ResMut<NextState<IsMainMenuShown>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (entity, transform) = *unit;
    // B for "build village"
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        info!("Key pressed !");
        // build village !
        info!("Building village...");
        *village.0 = Visibility::Visible;
        village.1.translation = Vec3 {
            x: transform.translation.x,
            y: transform.translation.y,
            z: 90.,
        };
        commands.entity(entity.entity()).despawn();

        // Show "you win" Button
        next_state.set(AppState::WinConditionAchieved);
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        // Toggling Main Menu
        main_menu_next_state.set(main_menu_current_state.next());
    }
}
