use bevy::prelude::*;
use crate::GameState;

pub struct VictoryScreenPlugin;

impl Plugin for VictoryScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(
                OnEnter(GameState::Finished), 
                (
                    show,
                    start_timer
                )
            )
            .add_systems(
                Update, 
                update.run_if(in_state(GameState::Finished))
            )
            .add_systems(
                OnExit(GameState::Finished), 
                (
                    hide,
                )
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
                "Congratulations!",
                TextStyle {
                    font: asset_server.load("fonts/FreeSans.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ).with_alignment(TextAlignment::Center),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(VictoryUi)
        .insert(VictoryTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn show(
    mut query: Query<
        &mut Visibility,
        With<VictoryUi>>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
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
    mut state: ResMut<NextState<GameState>>,
) {
    let mut timer = query.single_mut();
    timer.0.tick(time.delta());

    if timer.0.percent_left() < 0.001 {
        state.set(GameState::MainMenu);
    }
}

fn hide(
    mut query: Query<
        &mut Visibility,
        With<VictoryUi>>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}