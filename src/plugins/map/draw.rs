use std::process::exit;

use bevy::{color::palettes::css::RED, input_focus::InputFocus, prelude::*};

use crate::plugins::{
    PRESSED_BUTTON, SpriteAtlas, TerrainLayer,
    map::{Map, MapCoordinates, MapResource, Tile},
    select_on_click,
};

#[derive(Component)]
struct EndTurnButton;

pub fn draw_map(
    mut commands: Commands,
    assets: Res<AssetServer>,
    atlas: Res<SpriteAtlas>,
    map_resource: Res<MapResource>,
) {
    info!("Drawing map…");

    let map = &map_resource.map;
    for (index, tile) in map.iter().enumerate() {
        debug!("Map index: {index}");
        let map_coordinates: MapCoordinates = index.into();
        debug!("Map coordinates: {map_coordinates}");
        let world_position: Vec2 = map_coordinates.into();
        debug!("World Position: {world_position}");

        let terrain_variant = get_terrain_variant(tile, map, &map_coordinates);
        let z: f32 = match tile.terrain {
            TerrainLayer::Debug => 0.,
            TerrainLayer::Desert => 1.,
            TerrainLayer::Plain => 2.,
            TerrainLayer::Ocean => 3.,
        };
        commands
            .spawn((
                atlas.sprite(&tile.terrain.get_sprite_type(), terrain_variant, None),
                Transform {
                    translation: world_position.extend(z + (index as f32 / 10000.)),
                    ..default()
                },
                Pickable::default(),
            ))
            .observe(select_on_click);

        if let Some(zone) = tile.zone {
            let zone_variant = get_zone_variant(tile, map, &map_coordinates);

            commands
                .spawn((
                    atlas.sprite(&zone.get_sprite_type(), zone_variant, None),
                    Transform {
                        translation: world_position.extend(10.),
                        ..default()
                    },
                    Pickable::default(),
                ))
                .observe(select_on_click);
        }

        if let Some(feature) = tile.feature {
            commands
                .spawn((
                    atlas.sprite(&feature.get_sprite_type(), terrain_variant, None),
                    Transform {
                        translation: world_position.extend(20.),
                        ..default()
                    },
                    Pickable::default(),
                ))
                .observe(select_on_click);
        }
    }

    info!("Done drawing map !");

    info!("Drawing Map UI…");

    // Spawn turn counter
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            top: percent(-45),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Pickable::IGNORE,
        children![(
            Node {
                width: px(60),
                height: px(30),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(px(2)),
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::BLACK),
            children![(
                Text::new("Turn: 0"),
                TextFont {
                    font: assets.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default()
            )]
        )],
    ));

    // Spawn turn counter
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            top: percent(45),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Pickable::IGNORE,
        children![(
            Button,
            Pickable::default(),
            EndTurnButton,
            Node {
                width: px(60),
                height: px(30),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(px(2)),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::BLACK),
            children![(
                Text::new("End Turn"),
                TextFont {
                    font: assets.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default()
            )]
        )],
    ));

    info!("Done drawing Map UI !");
}

fn on_end_turn_button_click(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
            &Children,
        ),
        (With<EndTurnButton>, Changed<Interaction>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (entity, interaction, mut color, mut border_color, mut button, children) in
        interaction_query
    {
        let mut text = text_query.get_mut(children[0]).unwrap();

        if *interaction == Interaction::Pressed {
            input_focus.set(entity);
            **text = "Bye".to_string();
            *color = PRESSED_BUTTON.into();
            *border_color = BorderColor::all(RED);
            button.set_changed();
            exit(0);
        }
    }
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
pub fn get_terrain_variant(tile: &Tile, map: &Map, map_coordinates: &MapCoordinates) -> u8 {
    let terrain = tile.terrain;
    let Neighbours {
        bottom,
        bottom_left,
        bottom_right,
        left,
        right,
        top,
        top_left,
        top_right,
    } = get_neighbours(map, map_coordinates);

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
    // or Some(terrain) which is the "background" tile on top of which a partial tile
    // will be applied (think an ocean shore on top of a plain to make a beach).
    match (
        top_left.terrain == terrain,
        top.terrain == terrain,
        top_right.terrain == terrain,
        left.terrain == terrain,
        right.terrain == terrain,
        bottom_left.terrain == terrain,
        bottom.terrain == terrain,
        bottom_right.terrain == terrain,
    ) {
        // Regular corners
        (_, false, _, false, true, _, true, true) => 0,
        (_, false, _, true, false, true, true, _) => 2,
        (_, true, true, false, true, _, false, _) => 14,
        (true, true, _, true, false, _, false, _) => 16,

        // Regular sides
        (_, true, true, false, true, _, true, true) => 7,
        (true, true, _, true, false, true, true, _) => 9,
        (_, false, _, true, true, true, true, true) => 1,
        (true, true, true, true, true, _, false, _) => 15,

        // 1-width tiles (with edges on either side)
        // Vertical
        (_, false, _, false, false, _, true, _) => 3,
        (_, true, _, false, false, _, true, _) => 10,
        (_, true, _, false, false, _, false, _) => 17,
        // Horizontal
        (_, false, _, false, true, _, false, _) => 21,
        (_, false, _, true, true, _, false, _) => 22,
        (_, false, _, true, false, _, false, _) => 23,

        // Single internal corners (without edges)
        (true, true, true, true, true, true, true, false) => 4,
        (true, true, true, true, true, false, true, true) => 5,
        (true, true, false, true, true, true, true, true) => 11,
        (false, true, true, true, true, true, true, true) => 12,

        // Single internal corners (with vertical edges)
        (_, true, true, false, true, _, true, false) => 28,
        (true, true, _, true, false, false, true, _) => 29,
        (_, true, false, false, true, _, true, true) => 35,
        (false, true, _, true, false, true, true, _) => 36,

        // Single internal corners (with horizontal edges)
        (_, false, _, true, true, true, true, false) => 30,
        (_, false, _, true, true, false, true, true) => 31,
        (true, true, false, true, true, _, false, _) => 37,
        (false, true, true, true, true, _, false, _) => 38,

        // Double internal corners (without edges)
        (false, true, false, true, true, true, true, true) => 6,
        (false, true, true, true, true, false, true, true) => 13,
        (true, true, false, true, true, true, true, false) => 20,
        (true, true, true, true, true, false, true, false) => 27,
        (true, true, false, true, true, false, true, true) => 44,
        (false, true, true, true, true, true, true, false) => 45,

        // Triple internal corners (without edges)
        (false, true, false, true, true, true, true, false) => 18,
        (false, true, true, true, true, false, true, false) => 19,
        (true, true, false, true, true, false, true, false) => 25,
        (false, true, false, true, true, false, true, true) => 26,

        // Corners + opposite internal corners
        (_, false, _, false, true, _, true, false) => 32,
        (_, false, _, true, false, false, true, _) => 34,
        (_, true, false, false, true, _, false, _) => 46,
        (false, true, _, true, false, _, false, _) => 48,

        // Edges + opposite internal corners
        (_, false, _, true, true, false, true, false) => 33,
        (_, true, false, false, true, _, true, false) => 39,
        (false, true, _, true, false, false, true, _) => 41,
        (false, true, false, true, true, _, false, _) => 47,

        // Center tiles (either isolated, with or without full corners, etc.)
        (true, true, true, true, true, true, true, true) => 8,
        (false, true, false, true, true, false, true, false) => 40,
        (_, _, _, _, _, _, _, _) => 24,
    }
}

pub fn get_zone_variant(tile: &Tile, map: &Map, map_coordinates: &MapCoordinates) -> u8 {
    let zone = tile.zone;

    let Neighbours {
        bottom,
        bottom_left,
        bottom_right,
        left,
        right,
        top,
        top_left,
        top_right,
    } = get_neighbours(map, map_coordinates);

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
    // or Some(zone) which is the "background" tile on top of which a partial tile
    // will be applied (think an ocean shore on top of a plain to make a beach).
    match (
        top_left.zone == zone,
        top.zone == zone,
        top_right.zone == zone,
        left.zone == zone,
        right.zone == zone,
        bottom_left.zone == zone,
        bottom.zone == zone,
        bottom_right.zone == zone,
    ) {
        // Regular corners
        (_, false, _, false, true, _, true, true) => 0,
        (_, false, _, true, false, true, true, _) => 2,
        (_, true, true, false, true, _, false, _) => 14,
        (true, true, _, true, false, _, false, _) => 16,

        // Regular sides
        (_, true, true, false, true, _, true, true) => 7,
        (true, true, _, true, false, true, true, _) => 9,
        (_, false, _, true, true, true, true, true) => 1,
        (true, true, true, true, true, _, false, _) => 15,

        // 1-width tiles (with edges on either side)
        // Vertical
        (_, false, _, false, false, _, true, _) => 3,
        (_, true, _, false, false, _, true, _) => 10,
        (_, true, _, false, false, _, false, _) => 17,
        // Horizontal
        (_, false, _, false, true, _, false, _) => 21,
        (_, false, _, true, true, _, false, _) => 22,
        (_, false, _, true, false, _, false, _) => 23,

        // Single internal corners (without edges)
        (true, true, true, true, true, true, true, false) => 4,
        (true, true, true, true, true, false, true, true) => 5,
        (true, true, false, true, true, true, true, true) => 11,
        (false, true, true, true, true, true, true, true) => 12,

        // Single internal corners (with vertical edges)
        (_, true, true, false, true, _, true, false) => 28,
        (true, true, _, true, false, false, true, _) => 29,
        (_, true, false, false, true, _, true, true) => 35,
        (false, true, _, true, false, true, true, _) => 36,

        // Single internal corners (with horizontal edges)
        (_, false, _, true, true, true, true, false) => 30,
        (_, false, _, true, true, false, true, true) => 31,
        (true, true, false, true, true, _, false, _) => 37,
        (false, true, true, true, true, _, false, _) => 38,

        // Double internal corners (without edges)
        (false, true, false, true, true, true, true, true) => 6,
        (false, true, true, true, true, false, true, true) => 13,
        (true, true, false, true, true, true, true, false) => 20,
        (true, true, true, true, true, false, true, false) => 27,
        (true, true, false, true, true, false, true, true) => 44,
        (false, true, true, true, true, true, true, false) => 45,

        // Triple internal corners (without edges)
        (false, true, false, true, true, true, true, false) => 18,
        (false, true, true, true, true, false, true, false) => 19,
        (true, true, false, true, true, false, true, false) => 25,
        (false, true, false, true, true, false, true, true) => 26,

        // Corners + opposite internal corners
        (_, false, _, false, true, _, true, false) => 32,
        (_, false, _, true, false, false, true, _) => 34,
        (_, true, false, false, true, _, false, _) => 46,
        (false, true, _, true, false, _, false, _) => 48,

        // Edges + opposite internal corners
        (_, false, _, true, true, false, true, false) => 33,
        (_, true, false, false, true, _, true, false) => 39,
        (false, true, _, true, false, false, true, _) => 41,
        (false, true, false, true, true, _, false, _) => 47,

        // Center tiles (either isolated, with or without full corners, etc.)
        (true, true, true, true, true, true, true, true) => 8,
        (false, true, false, true, true, false, true, false) => 40,
        (_, _, _, _, _, _, _, _) => 24,
    }
}

struct Neighbours<'a> {
    bottom: &'a Tile,
    bottom_left: &'a Tile,
    bottom_right: &'a Tile,
    left: &'a Tile,
    right: &'a Tile,
    top: &'a Tile,
    top_left: &'a Tile,
    top_right: &'a Tile,
}

fn get_neighbours<'a>(map: &'a Map, &MapCoordinates(w, h): &MapCoordinates) -> Neighbours<'a> {
    let default_tile = &Tile {
        feature: None,
        zone: None,
        terrain: TerrainLayer::Plain,
    };
    debug!("Getting Neighbours, {}, {}", w, h);
    let neighbours = Neighbours {
        top_left: map
            .get(&MapCoordinates(w.saturating_sub(1), h.saturating_add(1)))
            .unwrap_or(default_tile),
        top: map
            .get(&MapCoordinates(w, h.saturating_add(1)))
            .unwrap_or(default_tile),
        top_right: map
            .get(&MapCoordinates(w.saturating_add(1), h.saturating_add(1)))
            .unwrap_or(default_tile),
        left: map
            .get(&MapCoordinates(w.saturating_sub(1), h))
            .unwrap_or(default_tile),
        right: map
            .get(&MapCoordinates(w.saturating_add(1), h))
            .unwrap_or(default_tile),
        bottom_left: map
            .get(&MapCoordinates(w.saturating_sub(1), h.saturating_sub(1)))
            .unwrap_or(default_tile),
        bottom: map
            .get(&MapCoordinates(w, h.saturating_sub(1)))
            .unwrap_or(default_tile),
        bottom_right: map
            .get(&MapCoordinates(w.saturating_add(1), h.saturating_sub(1)))
            .unwrap_or(default_tile),
    };
    debug!("Done Getting Neighbours");
    neighbours
}
