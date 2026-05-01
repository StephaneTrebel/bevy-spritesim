use bevy::prelude::*;

use crate::{
    plugins::{SpriteAtlas, map::RealCoordinates},
    state::AppState,
};

#[derive(Component)]
pub struct SelectEntity;

#[derive(Component)]
pub struct Selector;

pub fn draw_selector(mut commands: Commands, atlas: Res<SpriteAtlas>) {
    info!("Drawing selector");
    commands.spawn((
        atlas.sprite(&crate::plugins::SpriteType::Selector, 0),
        Transform::from_xyz(0., 0., 0.0),
        Visibility::Hidden,
        Pickable::IGNORE,
        Selector,
    ));
}

pub fn select_tile(
    selected: Single<(Entity, &SelectEntity, &RealCoordinates)>,
    mut selector: Single<(&mut Visibility, &mut Transform), With<Selector>>,
    mut commands: Commands,
) {
    info!("Entity ({},{}) selected !", selected.2.x, selected.2.y);
    *selector.0 = Visibility::Visible;
    selector.1.translation = Vec3 {
        x: selected.2.x,
        y: selected.2.y,
        z: 100.,
    };
    commands.entity(selected.0).remove::<SelectEntity>();
}

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::ReadyToDraw), draw_selector);
        app.add_systems(PreUpdate, select_tile);
    }
}
