use bevy::prelude::*;

use crate::{direction::{Direction, keypress_to_direction}, Spritesheet, GRID_SIZE, world_plugin::WORLD_SIZE};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, setup)
            .add_system(show_which_keys_are_pressed);
    }
}

#[derive(Component)]
struct ControlsDisplay(Direction);

fn setup(
    mut commands: Commands,
    spritesheet: Res<Spritesheet>,
) {
    let display_centre = Vec3::new(-(WORLD_SIZE as f32 / 1.25 * GRID_SIZE), 0.0, 1.0);

    for direction in [Direction::Down, Direction::Up, Direction::Left, Direction::Right] {
        let position = display_centre + get_direction_key_display_offset(direction) * GRID_SIZE;
        commands
            .spawn()
            .insert(ControlsDisplay(direction))
            .insert_bundle(SpriteSheetBundle{
                texture_atlas: spritesheet.0.clone(),
                transform: Transform {
                    translation: position,
                    scale: Vec3::splat(super::PIXEL_SCALE),
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: get_direction_key_sprite_index(direction),
                    ..default()
                },
                ..default()
            });
    }
    
}

fn show_which_keys_are_pressed(
    keyboard_input: Res<Input<KeyCode>>,
    mut control_display_query: Query<(& ControlsDisplay, &mut TextureAtlasSprite)>,
) {
    let key_direction = keypress_to_direction(keyboard_input);

    for (control_display, mut sprite) in control_display_query.iter_mut() {
        sprite.index = get_direction_key_sprite_index(control_display.0);

        if key_direction.is_some() && control_display.0 == key_direction.unwrap() {
            sprite.index = get_direction_key_sprite_index(control_display.0) + 7;
        }
    }
}

fn get_direction_key_sprite_index(direction: Direction) -> usize {
    match direction {
        Direction::Up => return 56,
        Direction::Left => return 57,
        Direction::Right => return 58,
        Direction::Down => return 59,
    }
}

fn get_direction_key_display_offset(direction: Direction) -> Vec3 {
    match direction {
        Direction::Up => return Vec3::new(0.0, 1.0, 0.0),
        Direction::Left => return Vec3::new(-1.0, 0.0, 0.0),
        Direction::Right => return Vec3::new(1.0, 0.0, 0.0),
        Direction::Down => return Vec3::splat(0.0),
    }
}
