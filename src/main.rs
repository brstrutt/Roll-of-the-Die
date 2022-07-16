use bevy::{
    prelude::*, 
    log::LogPlugin, 
};

mod die_plugin;
mod sub_spritesheet;
mod world_plugin;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let logger_config = wasm_logger::Config::new(log::Level::Info);

    wasm_logger::init(logger_config);
    log::info!("App is starting");

    App::new()
        .add_plugins_with(DefaultPlugins, |group| group.disable::<LogPlugin>())
        .add_startup_system(setup_2d_display)
        .add_plugin(world_plugin::WorldPlugin)
        .add_plugin(die_plugin::DiePlugin)
        .run();
}

fn setup_2d_display(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn load_spritesheet(
    asset_server: & Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load("resources/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(GRID_PIXEL_COUNT), 7, 7);
    return texture_atlases.add(texture_atlas);
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct PressurePlate{
    activated: bool,
    number: usize,
}

// Globals
pub const PIXEL_SCALE: f32 = 4.0;
pub const GRID_PIXEL_COUNT: f32 = 11.0;
pub const GRID_SIZE: f32 = PIXEL_SCALE * GRID_PIXEL_COUNT;
pub const TIME_STEP: f32 = 1.0 / 60.0;