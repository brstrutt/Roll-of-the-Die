use bevy::prelude::*;

use crate::{direction::{Direction, keypress_to_direction}, Spritesheet, GRID_SIZE, world_plugin::WORLD_SIZE, die_plugin::{get_die_face_sprite_index, Die}};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, setup)
            .add_systems(Update, show_which_keys_are_pressed)
            .add_systems(Update, show_which_die_faces_are_adjacent);
    }
}

#[derive(Component)]
struct ControlsDisplay(Direction);

#[derive(Component)]
struct AdjacentFacesDisplay(Option<Direction>);

fn setup(
    mut commands: Commands,
    spritesheet: Res<Spritesheet>,
) {
    let display_centre = Vec3::new(-(WORLD_SIZE as f32 / 1.25 * GRID_SIZE), 0.0, 1.0);

    for direction in [
        None,
        Some(Direction::Down),
        Some(Direction::Up),
        Some(Direction::Left),
        Some(Direction::Right)] {
            if direction.is_some() {
                let direction = direction.unwrap();
                commands
                    .spawn((
                        ControlsDisplay(direction),
                        SpriteSheetBundle{
                            texture_atlas: spritesheet.0.clone(),
                            transform: Transform {
                                translation: display_centre + get_direction_key_display_offset(Some(direction)) * GRID_SIZE * 2.0,
                                scale: Vec3::splat(super::PIXEL_SCALE),
                                ..default()
                            },
                            sprite: TextureAtlasSprite {
                                index: get_direction_key_sprite_index(direction),
                                ..default()
                            },
                            ..default()
                        }
                    ));
            }

            commands
                .spawn((
                    AdjacentFacesDisplay(direction),
                    SpriteSheetBundle{
                        texture_atlas: spritesheet.0.clone(),
                        transform: Transform {
                            translation: display_centre + get_direction_key_display_offset(direction) * GRID_SIZE,
                            scale: Vec3::splat(super::PIXEL_SCALE),
                            ..default()
                        },
                        sprite: TextureAtlasSprite {
                            index: 0,
                            ..default()
                        },
                        ..default()
                    }
                ));
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
                Some(Direction::Up) => die.bottom_number,
                Some(Direction::Right) => die.left_number,
                Some(Direction::Down) => die.top_number,
                Some(Direction::Left) => die.right_number,
                None => die.face_number,
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

fn get_direction_key_display_offset(direction: Option<Direction>) -> Vec3 {
    match direction {
        Some(Direction::Up) => return Vec3::new(0.0, -1.0, 0.0),
        Some(Direction::Left) => return Vec3::new(1.0, 0.0, 0.0),
        Some(Direction::Right) => return Vec3::new(-1.0, 0.0, 0.0),
        Some(Direction::Down) => return Vec3::new(0.0, 1.0, 0.0),
        None => return Vec3::splat(0.0),
    }
}
