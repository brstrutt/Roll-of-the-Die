use std::ops::Sub;

use bevy::{
    prelude::*,
    math::const_vec3,
    core::FixedTimestep,
};

use crate::{Collider, GRID_SIZE, PressurePlate};

use super::sub_spritesheet::SubSpritesheet;
use super::direction::{
    *,
    Direction};

pub struct DiePlugin;

impl Plugin for DiePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(super::TIME_STEP as f64))
                    .with_system(move_die)
            );
    }
}

const DIE_STARTING_POSITION: Vec3 = const_vec3!([0.0, 0.0, 1.0]);
const MOVEMENT_COOLDOWN: f32 = 0.5;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(DieBundle::new(crate::load_spritesheet(&asset_server, &mut texture_atlases)));
}

#[derive(Component, Deref, DerefMut)]
struct MovementCooldown(Timer);

#[derive(Component)]
struct Die {
    face_number: usize,
    top_number: usize,
    right_number: usize,
    bottom_number: usize,
    left_number: usize,
    hidden_number: usize,
}

#[derive(Bundle)]
struct DieBundle {
    die: Die,
    collider: Collider,
    movement_cooldown: MovementCooldown,
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    sub_spritesheet: SubSpritesheet,
}

impl DieBundle {
    fn new(texture_atlas_handle: Handle<TextureAtlas>) -> DieBundle {
        let spritesheet_indices: Vec<usize> = vec![1,2,3,4,5,6];
        let initial_index = spritesheet_indices[0];
        DieBundle { 
            die: Die { 
                face_number: 1,
                top_number: 2,
                right_number: 3,
                bottom_number: 5,
                left_number: 4,
                hidden_number: 6,
            },
            collider: Collider,
            movement_cooldown: MovementCooldown(Timer::from_seconds(MOVEMENT_COOLDOWN, false)),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: DIE_STARTING_POSITION,
                    scale: Vec3::splat(super::PIXEL_SCALE),
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: initial_index,
                    ..default()
                },
                ..default()
            },
            sub_spritesheet: SubSpritesheet{ spritesheet_indices },
        }
    }
}


fn move_die(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut die_query: Query<(&mut Transform, &mut TextureAtlasSprite, &mut SubSpritesheet, &mut MovementCooldown, &mut Die)>,
    colliders_query: Query<
        & Transform,
        (With<Collider>,Without<Die>),
    >,
    mut pressure_plates_query: Query<
        (&mut PressurePlate, &mut TextureAtlasSprite, &Transform),
        (Without<Die>, Without<Collider>),
    >,
) {
    let (
        mut transform,
        mut sprite,
        sub_spritesheet,
        mut movement_cooldown,
        mut die,) = die_query.single_mut();
    
    movement_cooldown.tick(time.delta());
    if movement_cooldown.finished() {
        let direction = keypress_to_direction(keyboard_input);

        if direction.is_none() { return; }
        let direction = direction.unwrap();
        let new_position = transform.translation + (translation_from_direction(&direction) * Vec3::splat(super::GRID_SIZE));

        for collider in colliders_query.iter() {
            if is_colliding(new_position, collider.translation) {
                return;
            }
        }

        transform.translation = new_position;

        rotate_die(&mut die, &direction);
        sprite.index = sub_spritesheet.get_sprite_index(die.face_number - 1);
        movement_cooldown.reset();

        for (mut pressure_plate, mut texture_atlas_sprite, pp_transform) in pressure_plates_query.iter_mut() {
            if is_colliding(new_position, pp_transform.translation) && 
                !pressure_plate.activated {
                if die.face_number == pressure_plate.number {
                    texture_atlas_sprite.index -= 7;
                    pressure_plate.activated = true;
                    sprite.index = sub_spritesheet.get_sprite_index(die.face_number - 1) + 14;
                }
                else {
                    sprite.index = sub_spritesheet.get_sprite_index(die.face_number - 1) + 7;
                }
            }
        }
    }
}

fn is_colliding(object1_pos: Vec3, object2_pos: Vec3) -> bool {
    let difference = object1_pos.sub(object2_pos);
    return difference.length().abs() < GRID_SIZE; // Just do sphere collision detection because everything is squares
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
