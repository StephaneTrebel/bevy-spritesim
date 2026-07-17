use bevy::math::{Vec2, Vec3};

use crate::plugins::{
    FeatureLayer, MAP_HEIGHT, MAP_WIDTH, SPRITE_DISPLAY_SIZE, TerrainLayer, ZoneLayer,
};

/// Coordinates on the "business logic" map which is stored
/// as a continuous list of Tiles
#[derive(Debug, Clone, Copy)]
pub struct MapCoordinates(pub u16, pub u16);

impl std::fmt::Display for MapCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

/// Convert a tile index into the related `MapCoordinates`
impl From<usize> for MapCoordinates {
    fn from(index: usize) -> Self {
        Self((index as u16) % MAP_WIDTH, (index as u16) / MAP_WIDTH)
    }
}

/// Convert a `MapCoordinates` to a `Vec2` (world coordinates)
impl From<MapCoordinates> for Vec2 {
    fn from(MapCoordinates(w, h): MapCoordinates) -> Self {
        Vec2 {
            x: f32::from(w * SPRITE_DISPLAY_SIZE),
            y: f32::from(h * SPRITE_DISPLAY_SIZE),
        }
    }
}

/// Convert a Vec3 (world 3D coordinates) to a MapCoordinates
impl From<Vec3> for MapCoordinates {
    fn from(Vec3 { x, y, z }: Vec3) -> Self {
        Self(
            ((x + f32::from(SPRITE_DISPLAY_SIZE / 2)) as u16) / SPRITE_DISPLAY_SIZE,
            ((y + f32::from(SPRITE_DISPLAY_SIZE / 2)) as u16) / SPRITE_DISPLAY_SIZE,
        )
    }
}

/// Convert a Vec2 (world coordinates) to a MapCoordinates
impl From<Vec2> for MapCoordinates {
    fn from(Vec2 { x, y }: Vec2) -> Self {
        Self(
            ((x + f32::from(SPRITE_DISPLAY_SIZE / 2)) as u16) / SPRITE_DISPLAY_SIZE,
            ((y + f32::from(SPRITE_DISPLAY_SIZE / 2)) as u16) / SPRITE_DISPLAY_SIZE,
        )
    }
}

/// A «Tile» is a superposition of several things that will compose the Map.
///
/// A Tile is made of several layers, from bottom to top (only the first one is
/// mandatory, the other are all optional):
/// - a Terrain (Plain, Desert, Ocean, etc.)
/// - a Zone (Forest, Hill, Mountain, etc.)
/// - a Feature (Food, Ore, Road, etc.)
/// - a Unit (Settler, Soldier, Wagon, etc.) that is moving through it
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

    /// Check whether a unit is allowed to move to a target coordinates.
    pub(crate) fn is_movement_allowed(&self, map_coordinates: MapCoordinates) -> bool {
        if map_coordinates.0 > MAP_WIDTH - 1 || map_coordinates.1 > MAP_HEIGHT - 1 {
            return false;
        }
        let tile = self
            .get(&map_coordinates)
            .expect("Coordinates should exist in map");
        tile.terrain != TerrainLayer::Ocean && tile.zone != Some(ZoneLayer::Mountain)
    }
}
