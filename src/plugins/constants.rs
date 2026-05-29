pub const WINDOW_PHYSICAL_WIDTH: u32 = 1280; // In pixels
pub const WINDOW_PHYSICAL_HEIGHT: u32 = 1280; // In pixels
pub const WINDOW_SCALE_FACTOR: f32 = 2.; // How much tiles are streched out in the beginning

pub const SPRITE_DISPLAY_SIZE: u16 = 16; // Size of a sprite side length, in pixels when drawn
pub const VARIANT_COUNT: u8 = 49; // Sprite variant count (all the different shapes a tile can be)

// Map dimension (in tiles)
pub const MAP_HEIGHT: u16 = 200;
pub const MAP_WIDTH: u16 = 200;

// Map center inside camera frustrum
pub const W_OFFSET: f32 = SPRITE_DISPLAY_SIZE * (MAP_WIDTH as f32) / 2.;
pub const H_OFFSET: f32 = SPRITE_DISPLAY_SIZE * (MAP_HEIGHT as f32) / 2.;
