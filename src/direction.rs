use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub fn translation_from_direction(direction: &Direction) -> Vec3 {
    match direction {
        Direction::Up => return Vec3::new(0.0, 1.0, 0.0),
        Direction::Down => return Vec3::new(0.0, -1.0, 0.0),
        Direction::Left => return Vec3::new(-1.0, 0.0, 0.0),
        Direction::Right => return Vec3::new(1.0, 0.0, 0.0),
    }
}

pub fn keypress_to_direction(keyboard_input: Res<Input<KeyCode>>) -> Option<Direction> {
    if keyboard_input.pressed(KeyCode::Left) { return Some(Direction::Left); }
    if keyboard_input.pressed(KeyCode::Right) { return Some(Direction::Right); }
    if keyboard_input.pressed(KeyCode::Up) { return Some(Direction::Up); }
    if keyboard_input.pressed(KeyCode::Down) { return Some(Direction::Down); }
    None
}