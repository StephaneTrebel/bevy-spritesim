use std::process::exit;

use bevy::{color::palettes::basic::*, input_focus::InputFocus, prelude::*};

use crate::state::AppState;

pub struct ButtonsPlugin;
impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputFocus>()
            .add_systems(OnEnter(AppState::WinConditionAchieved), show_winning_button)
            .add_systems(Update, on_button_click);
    }
}

const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button(assets: &AssetServer) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                width: px(150),
                height: px(65),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(px(2)),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::BLACK),
            children![(
                Text::new("You won !"),
                TextFont {
                    font: assets.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default()
            )]
        )],
    )
}

fn show_winning_button(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(button(&assets));
}

fn on_button_click(
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
        Changed<Interaction>,
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
