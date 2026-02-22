#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TerrainType {
    Debug,
    Desert,
    Plain,
}

/// Biomes are above terrain and characterize a place
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BiomeType {
    Forest,
    Hill,
    Mountain,
    Ocean,
}

/// Terrain are the base layers of all tiles
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpecialType {
    Corn,
    Fish,
    Lumber,
    Ore,
    Snow,
}
