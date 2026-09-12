use bevy::prelude::*;

use crate::plugins::main_menu::state::IsMainMenuShown;

#[derive(Component, Default, Clone)]
pub struct MainMenuComponent;

#[derive(Component, Default, Clone)]
pub struct ExitButton;

#[derive(Component, Default, Clone)]
pub struct ExitButtonText;

fn spawn_exit_button() -> impl Scene {
    bsn! {
        Name::new("Exit button Scene")
        Button
        ExitButton
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
            Name::new("Exit button text")
            Text::new("Exit")
            ExitButtonText
            TextFont {
                font_size: FontSize::Px(13.0),
            }
            TextColor(Color::srgb(0.9, 0.9, 0.9))
        )]
    }
}

pub fn on_exit_button_click(
    mut commands: Commands,
    interaction_query: Single<&Interaction, (With<ExitButton>, Changed<Interaction>)>,
) {
    let interaction = interaction_query.into_inner();
    if *interaction == Interaction::Pressed {
        info!("Exit button click !");
        commands.write_message(AppExit::Success);
    }
}

fn spawn_main_menu_layout() -> impl Scene {
    bsn! {
        DespawnOnExit::<IsMainMenuShown>(IsMainMenuShown::ShowMenu)
        MainMenuComponent
        Name::new("Main Menu UI Layout")
        Node {
            width: percent(100),
            height: percent(100),
            padding: UiRect::all(px(25)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center
        }
        Pickable::IGNORE
        Children [
           spawn_exit_button()
        ]
    }
}

fn spawn_main_menu(mut commands: Commands) {
    info!("Spawning Main Menu…");
    commands.spawn_scene(spawn_main_menu_layout());
    info!("Done spawning Main Menu !");
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(IsMainMenuShown::ShowMenu),
            spawn_main_menu.run_if(in_state(IsMainMenuShown::ShowMenu)),
        );
        app.add_systems(
            Update,
            on_exit_button_click.run_if(in_state(IsMainMenuShown::ShowMenu)),
        );
    }
}
