use bevy::prelude::*;
use crate::{GameState, direction::keypress_to_direction};

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu)
                    .with_system(show_main_menu),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(update_main_menu)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu)
                    .with_system(hide_main_menu),
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
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Roll the Die!",
                TextStyle {
                    font: asset_server.load("fonts/FreeSans.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            visibility: Visibility{ is_visible: false },
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
        visibility.is_visible = true;
    }
}

fn update_main_menu(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    let direction = keypress_to_direction(keyboard_input);

    if direction.is_some() {
        let _ = state.overwrite_set(GameState::Playing);
    }
}

fn hide_main_menu(
    mut query: Query<
        &mut Visibility,
        With<MenuUi>>,
) {
    for mut visibility in query.iter_mut() {
        visibility.is_visible = false;
    }
}