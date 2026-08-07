use bevy::{
    app::{App, Plugin},
    ecs::{
        resource::Resource,
        system::{Commands, ResMut},
    },
    state::state::{NextState, OnEnter},
};

use crate::{
    plugins::map::{Map, draw_map, generator::generate_map},
    state::AppState,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MapGenerationStart), setup_map);
        app.add_systems(OnEnter(AppState::ReadyToDraw), draw_map);
    }
}

#[derive(Resource)]
pub struct MapResource {
    pub map: Map,
}

fn setup_map(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    commands.insert_resource(MapResource {
        map: generate_map(),
    });

    next_state.set(AppState::ReadyToDraw);
}
