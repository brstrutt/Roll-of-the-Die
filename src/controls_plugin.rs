use bevy::prelude::*;

use crate::{direction::{Direction, keypress_to_direction}, Spritesheet, GRID_SIZE, world_plugin::WORLD_SIZE, die_plugin::{get_die_face_sprite_index, Die}};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, setup)
            .add_system(show_which_keys_are_pressed)
            .add_system(show_which_die_faces_are_adjacent);
    }
}

#[derive(Component)]
struct ControlsDisplay(Direction);

#[derive(Component)]
struct AdjacentFacesDisplay(Direction);

fn setup(
    mut commands: Commands,
    spritesheet: Res<Spritesheet>,
) {
    let display_centre = Vec3::new(-(WORLD_SIZE as f32 / 1.25 * GRID_SIZE), 0.0, 1.0);

    for direction in [Direction::Down, Direction::Up, Direction::Left, Direction::Right] {
        commands
            .spawn()
            .insert(ControlsDisplay(direction))
            .insert_bundle(SpriteSheetBundle{
                texture_atlas: spritesheet.0.clone(),
                transform: Transform {
                    translation: display_centre + get_direction_key_display_offset(direction) * GRID_SIZE,
                    scale: Vec3::splat(super::PIXEL_SCALE),
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: get_direction_key_sprite_index(direction),
                    ..default()
                },
                ..default()
            });

        commands
            .spawn()
            .insert(AdjacentFacesDisplay(direction))
            .insert_bundle(SpriteSheetBundle{
                texture_atlas: spritesheet.0.clone(),
                transform: Transform {
                    translation: display_centre + get_direction_key_display_offset(direction) * GRID_SIZE * 2.0,
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

fn show_which_die_faces_are_adjacent(
    die_query: Query<&Die>,
    mut adjacent_faces_display_query: Query<(& AdjacentFacesDisplay, &mut TextureAtlasSprite)>,
) {
    let die = die_query.single();
    
    for (face_display, mut sprite) in adjacent_faces_display_query.iter_mut() {
        sprite.index = get_die_face_sprite_index(
            match face_display.0 {
                Direction::Up => die.top_number,
                Direction::Right => die.right_number,
                Direction::Down => die.bottom_number,
                Direction::Left => die.left_number,
            });
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
        Direction::Down => return Vec3::new(0.0, -1.0, 0.0),
    }
}
