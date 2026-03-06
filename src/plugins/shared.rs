use crate::plugins::SpriteType;

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
