use bevy::platform::collections::HashMap;

use crate::plugins::{BiomeType, SpecialType, TerrainType};

/// A Tile is made of several layers, from bottom to top (only the first one is
/// mandatory, the other are all optional):
/// - A base Terrain (Plain, Desert, etc.)
/// - a Biome (Forest, Hills, etc.)
/// - a Special characteristic (Food, Ore, Silver, etc.)
/// - a Development (Road, Farmland, etc.) TODO
/// - a Settlement (Village, Fort, etc.) TODO
/// - a Unit (Settler, Canon, etc.) that is moving through it TODO
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Layer {
    Terrain,
    Biome,
    Special,
}

/// This is a union of all sprites types. Used for using common sprite
/// drawing functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    Terrain(TerrainType),
    Biome(BiomeType),
    Special(SpecialType),
}

/// Retrieve the concrete Kind of a tile on a given Layer
/// TODO: Convert to Kind method
pub fn get_layer_from_kind(kind: &Kind) -> Layer {
    match kind {
        Kind::Terrain(_) => Layer::Terrain,
        Kind::Biome(_) => Layer::Biome,
        Kind::Special(_) => Layer::Special,
    }
}

/// In-memory map for all layers of a Tile
pub type TileLayers = HashMap<Layer, Kind>;

/// A «Tile» is a superposition of several things that will compose the Map.
#[derive(Debug, Clone)]
pub struct Tile {
    pub(crate) layers: TileLayers,

    // These are called «real» coordinates because they are not the coordinates
    // in the map, but rather are the coordinates of where the sprite will be drawn
    pub(crate) _real_coordinates: (f32, f32),
}

/// In-memory map for all gameplay and render purposes.
/// This is the heart of the game.
pub type Map = HashMap<(i32, i32), Tile>;
