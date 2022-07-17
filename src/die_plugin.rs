use std::ops::Sub;

use bevy::{
    prelude::*,
    math::const_vec3,
};

use crate::{Collider, GRID_SIZE, PressurePlate, Spritesheet, GameState, PIXEL_SCALE};

use super::direction::{
    *,
    Direction};

pub struct DiePlugin;

impl Plugin for DiePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, setup)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(react_to_input)
                    .with_system(tick_animation)
                    .with_system(tick_motion)
                    .with_system(new_check_pressure_plates)
            );
    }
}

pub const DIE_STARTING_POSITION: Vec3 = const_vec3!([0.0, 0.0, 1.0]);
const DIE_SPEED: f32 = PIXEL_SCALE * GRID_SIZE / 1.5; // SMaller is faster. Dunno why

fn setup(
    mut commands: Commands,
    spritesheet: Res<Spritesheet>,
) {
    commands.spawn_bundle(DieBundle::new(&spritesheet.0));
}

#[derive(PartialEq)]
enum DieAnimation {
    None,
    Frame1,
    Frame2,
    Frame3,
}

#[derive(Component)]
pub struct Die {
    pub face_number: usize,
    pub top_number: usize,
    pub right_number: usize,
    pub bottom_number: usize,
    pub left_number: usize,
    pub hidden_number: usize,
    animation_state: DieAnimation,
    pub animation_direction: Direction,
    pub destination_translation: Vec3, // Store tile index, NOT absolute position
}

#[derive(Bundle)]
struct DieBundle {
    die: Die,
    collider: Collider,
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

impl DieBundle {
    fn new(texture_atlas_handle: &Handle<TextureAtlas>) -> DieBundle {
        DieBundle { 
            die: Die { 
                face_number: 1,
                top_number: 2,
                right_number: 3,
                bottom_number: 5,
                left_number: 4,
                hidden_number: 6,
                animation_state: DieAnimation::None,
                animation_direction: Direction::Up,
                destination_translation: DIE_STARTING_POSITION / GRID_SIZE,
            },
            collider: Collider,
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform {
                    translation: DIE_STARTING_POSITION,
                    scale: Vec3::splat(super::PIXEL_SCALE),
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: get_die_face_sprite_index(1),
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn react_to_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut die_query: Query<&mut Die>,
    colliders_query: Query<
        & Transform,
        (With<Collider>,Without<Die>),
    >
) {
    let mut die = die_query.single_mut();
    if die.animation_state != DieAnimation::None { return; }
    
    let direction = keypress_to_direction(keyboard_input);
    if direction.is_none() { return; }
    let direction = direction.unwrap();

    let new_position = die.destination_translation + translation_from_direction(&direction);

    for collider in colliders_query.iter() {
        if is_colliding(new_position * GRID_SIZE, collider.translation) {
            return;
        }
    }

    die.destination_translation = new_position;
    rotate_die(&mut die, &direction);

    die.animation_state = DieAnimation::Frame1;
    die.animation_direction = direction.clone();
}

fn new_check_pressure_plates(
    mut die_query: Query<(&Transform, &Die, &mut TextureAtlasSprite)>,
    mut pressure_plates_query: Query<(&mut PressurePlate, &Transform)>
) {
    let (die_transform, die, mut sprite) = die_query.single_mut();
    for (mut pressure_plate, pp_transform) in pressure_plates_query.iter_mut() {
        if is_colliding(die_transform.translation, pp_transform.translation) &&
        (die.animation_state == DieAnimation::None ||  die.animation_state == DieAnimation::Frame3) {
            if die.face_number == pressure_plate.number {
                pressure_plate.activated = true;
                if die.animation_state == DieAnimation::None { sprite.index = get_die_face_sprite_index(die.face_number) + 14};
            }
            else {
                if die.animation_state == DieAnimation::None { sprite.index = get_die_face_sprite_index(die.face_number) + 7};
            }
        }
    }
}

fn tick_animation(
    mut die_query: Query<(& Transform, &mut TextureAtlasSprite, &mut Die)>,
) {
    let (
        transform,
        mut sprite,
        mut die) = die_query.single_mut();
    if die.animation_state == DieAnimation::None { return; }

    let dist_to_dest = die.destination_translation * GRID_SIZE - transform.translation;

    match dist_to_dest.length().abs() {
        40.0.. => {
            die.animation_state = DieAnimation::Frame1;
            sprite.index = get_die_face_sprite_index(die.face_number);
        },
        28.0..=40.0 => {
            die.animation_state = DieAnimation::Frame1;
            sprite.index = get_die_animation_frame_index(1, die.animation_direction);
        },
        16.0..=28.0 => {
            die.animation_state = DieAnimation::Frame2;
            sprite.index = get_die_animation_frame_index(2, die.animation_direction);
        },
        4.0..=16.0 => {
            die.animation_state = DieAnimation::Frame3;
            sprite.index = get_die_animation_frame_index(3, die.animation_direction);
        },
        _ => {
            die.animation_state = DieAnimation::None;
            sprite.index = get_die_face_sprite_index(die.face_number);
        },

    }
}

fn tick_motion(
    time: Res<Time>,
    mut die_query: Query<(&mut Transform, &Die)>,
) {
    let (mut die_transform, die) = die_query.single_mut();

    let diff = (die.destination_translation * GRID_SIZE) - die_transform.translation;
    let travel_direction = diff/diff.length().abs();

    let mut velocity = (travel_direction * GRID_SIZE) / (DIE_SPEED * time.delta().as_secs_f32());
    if velocity.length() > diff.length() {
        velocity = velocity * diff.length()/velocity.length(); // Limit velocity so you cannot overshoot
    }
    die_transform.translation += velocity;
    if die.animation_state == DieAnimation::None { 
        die_transform.translation = die.destination_translation * GRID_SIZE;
    }
}


fn is_colliding(object1_pos: Vec3, object2_pos: Vec3) -> bool {
    let difference = object1_pos.sub(object2_pos);
    return difference.length().abs() < GRID_SIZE / 2.0; // Just do sphere collision detection because everything is squares
}

fn rotate_die(die: &mut Die, rotation: &Direction) {
    match rotation {
        Direction::Up => {
            let old_face_number = die.face_number;
            die.face_number = die.bottom_number;
            die.bottom_number = die.hidden_number;
            die.hidden_number = die.top_number;
            die.top_number = old_face_number;
        },
        Direction::Right => {
            let old_face_number = die.face_number;
            die.face_number = die.left_number;
            die.left_number = die.hidden_number;
            die.hidden_number = die.right_number;
            die.right_number = old_face_number;
        },
        Direction::Down => {
            let old_face_number = die.face_number;
            die.face_number = die.top_number;
            die.top_number = die.hidden_number;
            die.hidden_number = die.bottom_number;
            die.bottom_number = old_face_number;
        },
        Direction::Left => {
            let old_face_number = die.face_number;
            die.face_number = die.right_number;
            die.right_number = die.hidden_number;
            die.hidden_number = die.left_number;
            die.left_number = old_face_number;
        },
    };
}

fn get_die_animation_frame_index(frame_num: usize, movement_direction: Direction) -> usize {
    let frames: [usize; 3];
    match movement_direction {
        Direction::Up => frames = [52,53,54],
        Direction::Down => frames = [54,53,52], 
        Direction::Right => frames = [49,50,51], 
        Direction::Left => frames = [51,50,49],   
    }

    return frames[frame_num - 1];
}

pub fn get_die_face_sprite_index(face_num: usize) -> usize {
    match face_num {
        1 => return 1,
        2 => return 2,
        3 => return 3,
        4 => return 4,
        5 => return 5,
        6 => return 6,
        _ => return 0,
    }
}
