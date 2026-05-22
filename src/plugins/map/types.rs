use crate::plugins::{FeatureLayer, MAP_HEIGHT, MAP_WIDTH, TerrainLayer, ZoneLayer};

#[derive(Debug, Clone)]
pub struct MapCoordinates(pub u16, pub u16);

/// A «Tile» is a superposition of several things that will compose the Map.
///
/// A Tile is made of several layers, from bottom to top (only the first one is
/// mandatory, the other are all optional):
/// - a Terrain (Plain, Desert, Ocean, etc.)
/// - a Zone (Forest, Hill, Mountain, etc.)
/// - a Feature (Food, Ore, Road, etc.)
/// - a Unit (Settler, Soldier, Wagon, etc.) that is moving through it TODO
#[derive(Debug, Clone, Copy)]
pub struct Tile {
    // pub unit: Option<UnitLayer>
    pub feature: Option<FeatureLayer>,
    pub zone: Option<ZoneLayer>,
    pub terrain: TerrainLayer,
}

/// In-memory map for all gameplay and render purposes.
/// This is the heart of the game.
pub struct Map(Vec<Tile>);

impl Map {
    pub(crate) fn get(&self, MapCoordinates(w, h): &MapCoordinates) -> Option<&Tile> {
        self.0.get((w + h * MAP_WIDTH) as usize)
    }

    pub(crate) fn set(&mut self, MapCoordinates(w, h): &MapCoordinates, tile: Tile) {
        self.0[(w + h * MAP_WIDTH) as usize] = tile;
    }

    pub(crate) fn new() -> Self {
        let default_tile = Tile {
            feature: None,
            zone: None,
            terrain: TerrainLayer::Plain,
        };
        Self([default_tile; ((MAP_WIDTH + 1) * (MAP_HEIGHT + 1)) as usize].to_vec())
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<'_, Tile> {
        self.0.iter()
    }
}
