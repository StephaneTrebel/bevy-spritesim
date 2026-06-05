pub const WINDOW_PHYSICAL_WIDTH: u32 = 1280; // In pixels
pub const WINDOW_PHYSICAL_HEIGHT: u32 = 1280; // In pixels
pub const WINDOW_SCALE_FACTOR: f32 = 2.; // How much tiles are streched out in the beginning

pub const SPRITE_DISPLAY_SIZE: u16 = 16; // Size of a sprite side length, in pixels when drawn
pub const VARIANT_COUNT: u8 = 49; // Sprite variant count (all the different shapes a tile can be)

// Map dimension (in tiles)
pub const MAP_HEIGHT: u16 = 200;
pub const MAP_WIDTH: u16 = 200;

/// Tiny scale factor applied to every tile sprite so that adjacent quads
/// overlap by a sub-pixel amount.  Without this, GPU rasterisation rounding
/// at tile boundaries can leave single-pixel gaps (the classic "tile seam"
/// artifact).  The value is small enough to be invisible but large enough
/// to guarantee coverage at any zoom level.
pub const TILE_SCALE: f32 = 1.001;
