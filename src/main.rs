use bevy::{
    prelude::*, 
    log::LogPlugin, 
};

mod die_plugin;
mod world_plugin;
mod direction;

use direction::keypress_to_direction;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let logger_config = wasm_logger::Config::new(log::Level::Info);

    wasm_logger::init(logger_config);
    log::info!("App is starting");

    App::new()
        .add_plugins_with(DefaultPlugins, |group| group.disable::<LogPlugin>())
        .add_state(GameState::MainMenu)
        .add_startup_system(setup_2d_display)
        .add_startup_system(setup_main_menu)
        .add_plugin(world_plugin::WorldPlugin)
        .add_plugin(die_plugin::DiePlugin)
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
        )
        .run();
}

fn setup_2d_display(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

#[derive(Component)]
struct MenuUi;

fn setup_main_menu(
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
            visibility: Visibility{ is_visible: true },
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

fn load_spritesheet(
    asset_server: & Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load("resources/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(GRID_PIXEL_COUNT), 7, 8);
    return texture_atlases.add(texture_atlas);
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct PressurePlate{
    activated: bool,
    number: usize,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum GameState {
    MainMenu,
    Playing,
    GameCompleted,
}

// Globals
pub const PIXEL_SCALE: f32 = 4.0;
pub const GRID_PIXEL_COUNT: f32 = 11.0;
pub const GRID_SIZE: f32 = PIXEL_SCALE * GRID_PIXEL_COUNT;
pub const TIME_STEP: f32 = 1.0 / 60.0;