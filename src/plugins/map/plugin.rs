use bevy::{
    app::{App, Plugin, PreUpdate},
    ecs::{
        resource::Resource,
        system::{Commands, ResMut},
    },
    state::state::{NextState, OnEnter},
};

use crate::{
    plugins::map::{
        Map,
        draw::{draw_map_ui, on_end_turn_button_click},
        draw_map,
        generator::generate_map,
    },
    state::AppState,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MapGenerationStart), setup_map);
        app.add_systems(OnEnter(AppState::ReadyToDraw), draw_map);
        app.add_systems(OnEnter(AppState::ReadyToDraw), draw_map_ui);
        app.add_systems(PreUpdate, on_end_turn_button_click);
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
