use crate::plugins::{
    SpecialType,
    map::{Kind, Layer, Map, Tile},
};

/// Retrieve the related layer of a Kind
pub fn get_kind_of_tile_layer(tile: &Tile, layer: &Layer) -> Option<Kind> {
    tile.layers.get(layer).copied()
}
/// Retrieve the adequate tileset indices to properly display a tile.
///
/// Indeed, tiles can either be one in the center of a patch (hence the tileable
/// center tile will be used), or on the edge (maybe even in a corner), so a proper
/// algorithmic pass must done to ensure the proper tile is used.
///
/// Additionnaly if a «partial» tile (like a corner) is used, we have to add
/// an underlying tile to serve as background so for instance a beach is composed of
/// a plain (its shore) and the ocean (its beach) over it.
pub fn get_tiles_to_display(
    tile: &Tile,
    map: &Map,
    map_coordinates: &(i32, i32),
    layer: Layer,
) -> (usize, Option<Kind>) {
    let kind = get_kind_of_tile_layer(tile, &layer);

    let default_tile = (*tile).clone();

    match layer {
        l if l == Layer::Terrain
            || l == Layer::Biome
            || l == Layer::Special && kind == Some(Kind::Special(SpecialType::Ore)) =>
        {
            let top_left = get_kind_of_tile_layer(
                map.get(&(map_coordinates.0 - 1, map_coordinates.1 + 1))
                    .unwrap_or(&default_tile),
                &layer,
            );
            let top = get_kind_of_tile_layer(
                map.get(&(map_coordinates.0, map_coordinates.1 + 1))
                    .unwrap_or(&default_tile),
                &layer,
            );
            let top_right = get_kind_of_tile_layer(
                map.get(&(map_coordinates.0 + 1, map_coordinates.1 + 1))
                    .unwrap_or(&default_tile),
                &layer,
            );
            let left = get_kind_of_tile_layer(
                map.get(&(map_coordinates.0 - 1, map_coordinates.1))
                    .unwrap_or(&default_tile),
                &layer,
            );
            let right = get_kind_of_tile_layer(
                map.get(&(map_coordinates.0 + 1, map_coordinates.1))
                    .unwrap_or(&default_tile),
                &layer,
            );
            let bottom_left = get_kind_of_tile_layer(
                map.get(&(map_coordinates.0 - 1, map_coordinates.1 - 1))
                    .unwrap_or(&default_tile),
                &layer,
            );
            let bottom = get_kind_of_tile_layer(
                map.get(&(map_coordinates.0, map_coordinates.1 - 1))
                    .unwrap_or(&default_tile),
                &layer,
            );
            let bottom_right = get_kind_of_tile_layer(
                map.get(&(map_coordinates.0 + 1, map_coordinates.1 - 1))
                    .unwrap_or(&default_tile),
                &layer,
            );

            // The main algorithm relies on a truth table which determines a tileset index
            // to use based on the ones surrounding the current tile:
            //
            // top_left    | top      | top_right
            // left        | OUR TILE | right
            // bottom_left | bottom   | bottom_right
            //
            // Depending on the surround tile we use one of the 47 possible tiles which
            // encompass all possible arrangements of corners, edgeds, internal corners, etc.
            //
            // A second value is returned, which is either None (for regular «full» tiles),
            // or Some(kind) which is the "background" tile on top of which a partial tile
            // will be applied (think an ocean shore on top of a plain to make a beach).
            match (
                top_left == kind,
                top == kind,
                top_right == kind,
                left == kind,
                right == kind,
                bottom_left == kind,
                bottom == kind,
                bottom_right == kind,
            ) {
                // Regular corners
                (_, false, _, false, true, _, true, true) => (0, top),
                (_, false, _, true, false, true, true, _) => (2, top),
                (_, true, true, false, true, _, false, _) => (14, left),
                (true, true, _, true, false, _, false, _) => (16, right),

                // Regular sides
                (_, true, true, false, true, _, true, true) => (7, left),
                (true, true, _, true, false, true, true, _) => (9, right),
                (_, false, _, true, true, true, true, true) => (1, top),
                (true, true, true, true, true, _, false, _) => (15, bottom),

                // 1-width tiles (with edges on either side)
                // Vertical
                (_, false, _, false, false, _, true, _) => (3, top),
                (_, true, _, false, false, _, true, _) => (10, left),
                (_, true, _, false, false, _, false, _) => (17, right),
                // Horizontal
                (_, false, _, false, true, _, false, _) => (21, top),
                (_, false, _, true, true, _, false, _) => (22, top),
                (_, false, _, true, false, _, false, _) => (23, top),

                // Single internal corners (without edges)
                (true, true, true, true, true, true, true, false) => (4, bottom_right),
                (true, true, true, true, true, false, true, true) => (5, bottom_left),
                (true, true, false, true, true, true, true, true) => (11, top_right),
                (false, true, true, true, true, true, true, true) => (12, top_left),

                // Single internal corners (with vertical edges)
                (_, true, true, false, true, _, true, false) => (28, left),
                (true, true, _, true, false, false, true, _) => (29, right),
                (_, true, false, false, true, _, true, true) => (35, top_right),
                (false, true, _, true, false, true, true, _) => (36, top_left),

                // Single internal corners (with horizontal edges)
                (_, false, _, true, true, true, true, false) => (30, top),
                (_, false, _, true, true, false, true, true) => (31, top),
                (true, true, false, true, true, _, false, _) => (37, top_right),
                (false, true, true, true, true, _, false, _) => (38, top_left),

                // Double internal corners (without edges)
                (false, true, false, true, true, true, true, true) => (6, top_left),
                (false, true, true, true, true, false, true, true) => (13, top_left),
                (true, true, false, true, true, true, true, false) => (20, top_right),
                (true, true, true, true, true, false, true, false) => (27, bottom_right),
                (true, true, false, true, true, false, true, true) => (44, top_right),
                (false, true, true, true, true, true, true, false) => (45, top_left),

                // Triple internal corners (without edges)
                (false, true, false, true, true, true, true, false) => (18, top_left),
                (false, true, true, true, true, false, true, false) => (19, top_left),
                (true, true, false, true, true, false, true, false) => (25, top_right),
                (false, true, false, true, true, false, true, true) => (26, top_left),

                // Corners + opposite internal corners
                (_, false, _, false, true, _, true, false) => (32, top),
                (_, false, _, true, false, false, true, _) => (34, top),
                (_, true, false, false, true, _, false, _) => (46, top_right),
                (false, true, _, true, false, _, false, _) => (48, top_left),

                // Edges + opposite internal corners
                (_, false, _, true, true, false, true, false) => (33, top),
                (_, true, false, false, true, _, true, false) => (39, top_right),
                (false, true, _, true, false, false, true, _) => (41, top_left),
                (false, true, false, true, true, _, false, _) => (47, top_left),

                // Center tiles (either isolated, with or without full corners, etc.)
                (true, true, true, true, true, true, true, true) => (8, top_left),
                (false, true, false, true, true, false, true, false) => (40, top_left),
                (_, _, _, _, _, _, _, _) => (24, top), // "top" is always false in the default case
            }
        }
        _ => (0, None),
    }
}
