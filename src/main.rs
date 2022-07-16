use bevy::{
    prelude::*, 
    log::LogPlugin, 
    math::const_vec3,
    core::FixedTimestep,
};

mod hello_plugin;
mod blue_rect_plugin;
mod cycling_die_display_plugin;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let logger_config = wasm_logger::Config::new(log::Level::Info);

    wasm_logger::init(logger_config);
    log::debug!("App is starting");

    App::new()
        .add_plugins_with(DefaultPlugins, |group| group.disable::<LogPlugin>())
        .add_startup_system(setup_2d_display)
        //.add_plugin(hello_plugin::HelloPlugin)
        //.add_plugin(blue_rect_plugin::BlueRectPlugin)
        .add_plugin(cycling_die_display_plugin::CyclingDieDisplayPlugin)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_die),
        )
        .run();
}

fn setup_2d_display(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(DieBundle::new());
}

const PIXEL_SCALE: f32 = 4.0;
const TIME_STEP: f32 = 1.0 / 60.0;
const DIE_STARTING_POSITION: Vec3 = const_vec3!([0.0, 0.0, 1.0]);
const DIE_COLOR: Color = Color::rgb(0.5, 0.1, 0.1);
const DIE_SPEED: f32 = 10.0;

#[derive(Component)]
struct Die;

#[derive(Bundle)]
struct DieBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    die: Die,
}

impl DieBundle {
    fn new() -> DieBundle {
        DieBundle { 
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: DIE_STARTING_POSITION,
                    scale: Vec3::splat(PIXEL_SCALE),
                    ..default()
                },
                sprite: Sprite {
                    color: DIE_COLOR,
                    ..default()
                },
                ..default()
            },
            die: Die
        }
    }
}

fn move_die(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Die>>,
) {
    let mut die_transform = query.single_mut();
    let mut direction = Vec3::splat(0.0);

    if keyboard_input.pressed(KeyCode::Left) {
        direction[0] -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction[0] += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction[1] -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction[1] += 1.0;
    }

    die_transform.translation = die_transform.translation + (direction * Vec3::splat(DIE_SPEED) * Vec3::splat(TIME_STEP));
}