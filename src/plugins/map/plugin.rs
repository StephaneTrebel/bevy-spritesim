use bevy::app::{App, Plugin, Startup};

use crate::plugins::map::setup::setup;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
