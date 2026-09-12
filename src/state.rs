use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    // Startup state, origin of EVERYTHING !
    #[default]
    Startup,

    // Assets loading
    SpriteLoadStart,
    SpriteLoadInProgress,
    CreateSpriteAtlas,

    // Map
    MapGenerationStart,
    ReadyToDraw,

    // Game running
    InGame,

    // Winning Conditions
    WinConditionAchieved,
}
