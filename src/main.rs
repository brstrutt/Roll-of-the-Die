use bevy::{prelude::*, log::LogPlugin};

mod hello_plugin;
mod blue_rect_plugin;

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
        .add_startup_system(draw_die_face)
        .run();
}

fn setup_2d_display(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn draw_die_face(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("resources/spritesheet.png"),
        ..default()
    });
}