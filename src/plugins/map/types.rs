use bevy::platform::collections::HashMap;

use crate::plugins::{FeatureLayer, TerrainLayer, ZoneLayer};

/// A «Tile» is a superposition of several things that will compose the Map.
///
/// A Tile is made of several layers, from bottom to top (only the first one is
/// mandatory, the other are all optional):
/// - a Terrain (Plain, Desert, Ocean, etc.)
/// - a Zone (Forest, Hill, Mountain, etc.)
/// - a Feature (Food, Ore, Road, etc.)
/// - a Unit (Settler, Soldier, Wagon, etc.) that is moving through it TODO
#[derive(Debug, Clone)]
pub struct Tile {
    // pub unit: Option<UnitLayer>
    pub feature: Option<FeatureLayer>,
    pub zone: Option<ZoneLayer>,
    pub terrain: TerrainLayer,

    // These are called «real» coordinates because they are not the coordinates
    // in the map, but rather are the coordinates of where the sprite will be drawn
    pub real_coordinates: (f32, f32),
}

/// In-memory map for all gameplay and render purposes.
/// This is the heart of the game.
pub type Map = HashMap<(u16, u16), Tile>;
