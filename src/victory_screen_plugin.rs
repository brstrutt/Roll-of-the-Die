use bevy::prelude::*;
use crate::{GameState, direction::keypress_to_direction};

pub struct VictoryScreenPlugin;

impl Plugin for VictoryScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::on_enter(GameState::Finished)
                    .with_system(show)
                    .with_system(start_timer),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Finished)
                    .with_system(update)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Finished)
                    .with_system(hide),
            );
    }
}
#[derive(Component)]
struct VictoryUi;

#[derive(Component)]

struct VictoryTimer(Timer);

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
                "Congratulations!",
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
        .insert(VictoryUi)
        .insert(VictoryTimer(Timer::from_seconds(5.0, false)));
}

fn show(
    mut query: Query<
        &mut Visibility,
        With<VictoryUi>>,
) {
    for mut visibility in query.iter_mut() {
        visibility.is_visible = true;
    }
}

fn start_timer(
    mut query: Query<&mut VictoryTimer>,
) {
    let mut timer = query.single_mut();
    timer.0.reset();
}

fn update(
    time: Res<Time>,
    mut query: Query<&mut VictoryTimer>,
    mut state: ResMut<State<GameState>>,
) {
    let mut timer = query.single_mut();
    timer.0.tick(time.delta());

    if timer.0.finished() {
        state.overwrite_set(GameState::MainMenu);
    }
}

fn hide(
    mut query: Query<
        &mut Visibility,
        With<VictoryUi>>,
) {
    for mut visibility in query.iter_mut() {
        visibility.is_visible = false;
    }
}