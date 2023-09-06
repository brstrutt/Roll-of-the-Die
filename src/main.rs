use bevy::{
    prelude::*, 
    log::LogPlugin, 
};

mod die_plugin;
mod world_plugin;
mod direction;
mod controls_plugin;
mod title_screen_plugin;
mod victory_screen_plugin;

use die_plugin::Die;
use die_plugin::DIE_STARTING_POSITION;
use die_plugin::get_die_face_sprite_index;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let logger_config = wasm_logger::Config::new(log::Level::Info);

    wasm_logger::init(logger_config);
    log::info!("App is starting");

    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest())
            .build().disable::<LogPlugin>()
        )
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((
            title_screen_plugin::TitleScreenPlugin,
            victory_screen_plugin::VictoryScreenPlugin,
            world_plugin::WorldPlugin,
            die_plugin::DiePlugin,
            controls_plugin::ControlsPlugin
        ))
        .add_systems(
            Update,
            check_for_victory.run_if(in_state(GameState::Playing))
        )
        .add_systems(
            Update,
            reset_game.run_if(in_state(GameState::Finished))
        )
        .insert_resource(FixedTime::new_from_secs(0.03))
        .run();
}

#[derive(Component, Resource)]
struct Spritesheet(Handle<TextureAtlas>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("resources/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle, 
        Vec2::splat(GRID_PIXEL_COUNT), 
        7, 
        10, 
        None, 
        None);
    commands.insert_resource(Spritesheet(texture_atlases.add(texture_atlas)));
}


fn check_for_victory(
    pressure_plates_query: Query<& PressurePlate>,
    mut state: ResMut<NextState<GameState>>,
) {
    let mut all_plates_active = true;
    for pressure_plate in pressure_plates_query.iter() {
        all_plates_active = all_plates_active && pressure_plate.activated;
    }

    if all_plates_active {
        state.set(GameState::Finished);
    }
}

fn reset_game(
    mut pressure_plates_query: Query<&mut PressurePlate>,
    mut die_query: Query<(&mut Transform, &mut Die, &mut TextureAtlasSprite)>,
) {
    for mut pressure_plate in pressure_plates_query.iter_mut() { pressure_plate.activated = false; }
    let (mut die_transform, mut die, mut sprite) = die_query.single_mut();

    die_transform.translation = DIE_STARTING_POSITION;
    die.destination_translation = DIE_STARTING_POSITION;
    sprite.index = get_die_face_sprite_index(die.face_number);
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct PressurePlate{
    activated: bool,
    number: usize,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
    Finished,
}

// Globals
pub const PIXEL_SCALE: f32 = 4.0;
pub const GRID_PIXEL_COUNT: f32 = 11.0;
pub const GRID_SIZE: f32 = PIXEL_SCALE * GRID_PIXEL_COUNT;
pub const TIME_STEP: f32 = 1.0 / 60.0;