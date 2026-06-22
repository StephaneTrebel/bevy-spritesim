use crate::plugins::sprites::SpriteType;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TerrainLayer {
    Debug,
    Desert,
    Plain,
    Ocean,
}

impl TerrainLayer {
    pub fn get_sprite_type(&self) -> SpriteType {
        match self {
            TerrainLayer::Debug => SpriteType::Debug,
            TerrainLayer::Desert => SpriteType::Desert,
            TerrainLayer::Plain => SpriteType::Plain,
            TerrainLayer::Ocean => SpriteType::Ocean,
        }
    }
}

impl std::fmt::Display for TerrainLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TerrainLayer::Debug => "DEBUG",
                TerrainLayer::Desert => "DESERT",
                TerrainLayer::Plain => "PLAIN",
                TerrainLayer::Ocean => "OCEAN",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ZoneLayer {
    Forest,
    Hill,
    Mountain,
}

impl ZoneLayer {
    pub fn get_sprite_type(&self) -> SpriteType {
        match self {
            ZoneLayer::Forest => SpriteType::Forest,
            ZoneLayer::Hill => SpriteType::Hill,
            ZoneLayer::Mountain => SpriteType::Mountain,
        }
    }
}

impl std::fmt::Display for ZoneLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ZoneLayer::Forest => "FOREST",
                ZoneLayer::Hill => "HILL",
                ZoneLayer::Mountain => "MOUNTAIN",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FeatureLayer {
    Corn,
    Fish,
    Lumber,
    Ore,
    Snow,
}

impl FeatureLayer {
    pub fn get_sprite_type(&self) -> SpriteType {
        match self {
            FeatureLayer::Corn => SpriteType::Corn,
            FeatureLayer::Fish => SpriteType::Fish,
            FeatureLayer::Lumber => SpriteType::Lumber,
            FeatureLayer::Ore => SpriteType::Ore,
            FeatureLayer::Snow => SpriteType::Snow,
        }
    }
}

impl std::fmt::Display for FeatureLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FeatureLayer::Corn => "CORN",
                FeatureLayer::Fish => "FISH",
                FeatureLayer::Lumber => "LUMBER",
                FeatureLayer::Ore => "ORE",
                FeatureLayer::Snow => "SNOW",
            }
        )
    }
}
