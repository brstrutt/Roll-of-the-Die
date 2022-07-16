use bevy::{
    prelude::*, 
    log::LogPlugin, 
};

mod cycling_die_display_plugin;
mod die_plugin;

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
        .add_plugin(cycling_die_display_plugin::CyclingDieDisplayPlugin)
        .add_plugin(die_plugin::DiePlugin)
        .run();
}

fn setup_2d_display(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// Globals
pub const PIXEL_SCALE: f32 = 4.0;
pub const TIME_STEP: f32 = 1.0 / 60.0;