use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    SpriteLoadStart,
    SpriteLoadInProgress,
    CreateSpriteAtlas,
    MapGenerationStart,
    ReadyToDraw,
}
