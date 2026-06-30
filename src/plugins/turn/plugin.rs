use bevy::prelude::*;

#[derive(Resource)]
pub struct TurnResource {
    pub turn_count: u32,
}

fn create_turn_resource(mut commands: Commands) {
    info!("Creating Turn Resource");
    commands.insert_resource(TurnResource { turn_count: 0 });
}

pub struct TurnPlugin;
impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_turn_resource);
    }
}
