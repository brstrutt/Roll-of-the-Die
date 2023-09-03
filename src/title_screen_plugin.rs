use bevy::prelude::*;
use crate::{GameState, direction::keypress_to_direction};

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(
                OnEnter(GameState::MainMenu), 
                show_main_menu
            )
            .add_systems(
                Update, 
                update_main_menu.run_if(in_state(GameState::MainMenu))
            )
            .add_systems(
                OnExit(GameState::MainMenu), 
                hide_main_menu
            );
    }
}
#[derive(Component)]
struct MenuUi;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Roll the Die!",
                TextStyle {
                    font: asset_server.load("fonts/FreeSans.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ).with_alignment(TextAlignment::Center),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(MenuUi);
}

fn show_main_menu(
    mut query: Query<
        &mut Visibility,
        With<MenuUi>>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn update_main_menu(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    let direction = keypress_to_direction(keyboard_input);

    if direction.is_some() {
        state.set(GameState::Playing);
    }
}

fn hide_main_menu(
    mut query: Query<
        &mut Visibility,
        With<MenuUi>>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}