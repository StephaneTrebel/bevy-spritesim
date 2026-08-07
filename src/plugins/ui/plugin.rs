use bevy::{
    color::palettes::css::{RED, TEAL},
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};

use crate::{
    plugins::{turn::TurnResource, units::Unit},
    state::AppState,
};

#[derive(Component, Default, Clone)]
pub struct TurnCountText;

fn spawn_turn_counter() -> impl Scene {
    // Spawn turn counter
    bsn! {
        Name::new("Turn counter Scene")
        Node {
            width: px(100),
            height: px(30),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(px(2)),
            margin: UiRect::all(px(10)),
        }
        BorderColor::all(Color::WHITE)
        BackgroundColor(Color::BLACK)
        Children [(
            Name::new("Turn counter Text")
            Text::new(format!("Turn: 0"))
            TurnCountText
            TextFont {
                font_size: FontSize::Px(13.0),
            }
            TextColor(Color::srgb(0.9, 0.9, 0.9))
        )]
    }
}

#[derive(Component, Default, Clone)]
pub struct EndTurnButton;

#[derive(Component, Default, Clone)]
pub struct EndTurnButtonText;

fn spawn_end_turn_button() -> impl Scene {
    // Spawn turn counter
    bsn! {
        Name::new("End turn button Scene")
        Button
        EndTurnButton
        Node {
            width: px(100),
            height: px(30),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::MAX,
            margin: UiRect::all(px(10)),
        }
        BorderColor::all(Color::WHITE)
        BackgroundColor(Color::BLACK)
        Children [(
            Name::new("End turn button text")
            Text::new("End Turn")
            EndTurnButtonText
            TextFont {
                font_size: FontSize::Px(13.0),
            }
            TextColor(Color::srgb(0.9, 0.9, 0.9))
        )]
    }
}

fn spawn_ui_layout() -> impl Scene {
    bsn! {
        Name::new("UI Layout")
        Node {
            width: percent(100),
            height: percent(100),
            padding: UiRect::all(px(25)),
            align_items: AlignItems::End,
            justify_content: JustifyContent::End
        }
        Pickable::IGNORE
        Children [
           spawn_turn_counter(),
           spawn_end_turn_button()
        ]
    }
}

pub fn draw_map_ui(mut commands: Commands) {
    info!("Drawing Map UI…");
    commands.spawn_scene(spawn_ui_layout());
    info!("Done drawing Map UI !");
}

pub fn on_end_turn_button_click(
    mut input_focus: ResMut<InputFocus>,
    mut turn_resource: ResMut<TurnResource>,
    interaction_query: Single<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
        ),
        (With<EndTurnButton>, Changed<Interaction>),
    >,
    mut end_button_text_query: Single<&mut Text, (With<EndTurnButtonText>, Without<TurnCountText>)>,
    mut turn_count_text_query: Single<&mut Text, (With<TurnCountText>, Without<EndTurnButtonText>)>,
    units_query: Query<&mut Unit>,
) {
    let (entity, interaction, mut color, mut border_color, mut button) =
        interaction_query.into_inner();
    if *interaction == Interaction::Pressed {
        // Mark the button as clicked
        input_focus.set(entity, FocusCause::Pressed);
        ***end_button_text_query = "TURN ENDED".to_string();
        *color = TEAL.into();
        *border_color = BorderColor::all(RED);
        button.set_changed();

        // Increment count turn
        turn_resource.turn_count += 1;
        info!("TURN COUNT {}", turn_resource.turn_count);

        // Update turn count UI element
        **turn_count_text_query = format!("Turn: {}", turn_resource.turn_count).into();

        // Replenish MovementPoints for all units
        for mut unit in units_query {
            unit.movement_points = unit.movement_speed;
        }
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::ReadyToDraw), draw_map_ui);
        app.add_systems(PreUpdate, on_end_turn_button_click);
    }
}
