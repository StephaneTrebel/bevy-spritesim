use std::process::exit;

use bevy::{
    color::palettes::basic::RED,
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};

use crate::{plugins::PRESSED_BUTTON, state::AppState};

pub struct ButtonsPlugin;
impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputFocus>()
            .add_systems(OnEnter(AppState::WinConditionAchieved), show_winning_button)
            .add_systems(Update, on_winning_button_click);
    }
}

#[derive(Component)]
struct WinningButton;

fn show_winning_button(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            top: percent(-15),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            WinningButton,
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
                    font: assets.load("fonts/FiraSans-Bold.ttf").into(),
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default()
            )]
        )],
    ));
}

fn on_winning_button_click(
    mut input_focus: ResMut<InputFocus>,
    interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
            &Children,
        ),
        (With<WinningButton>, Changed<Interaction>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (entity, interaction, mut color, mut border_color, mut button, children) in
        interaction_query
    {
        let mut text = text_query.get_mut(children[0]).unwrap();

        if *interaction == Interaction::Pressed {
            input_focus.set(entity, FocusCause::Pressed);
            **text = "Bye".to_string();
            *color = PRESSED_BUTTON.into();
            *border_color = BorderColor::all(RED);
            button.set_changed();
            exit(0);
        }
    }
}
