use bevy::prelude::*;

use crate::{
    plugins::{SpriteAtlas, map::RealCoordinates},
    state::AppState,
};

#[derive(Component)]
struct SelectEntity;

#[derive(Component)]
struct Selector;

/// Draw the Selector sprite under the Map.
/// It will be then moved to a tile (above everything) when the latter is selected.
fn draw_selector(mut commands: Commands, atlas: Res<SpriteAtlas>) {
    info!("Drawing selector");
    commands.spawn((
        atlas.sprite(&crate::plugins::SpriteType::Selector, 0),
        Transform::from_xyz(0., 0., 0.0),
        Visibility::Hidden,
        Pickable::IGNORE,
        Selector,
    ));
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
}

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::ReadyToDraw), draw_selector)
            .add_systems(PreUpdate, select_tile);
    }
}
