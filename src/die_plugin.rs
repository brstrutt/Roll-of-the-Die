use bevy::{
    prelude::*,
    math::const_vec3,
    core::FixedTimestep,
};

use super::sub_spritesheet::SubSpritesheet;

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
struct Die;

#[derive(Bundle)]
struct DieBundle {
    die: Die,
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
            die: Die,
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
            sub_spritesheet: SubSpritesheet{ spritesheet_indices, current_index: 0 },
        }
    }
}

fn move_die(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<
        &mut Transform,
        With<Die>>,
    mut sprite_query: Query<
        &mut TextureAtlasSprite,
        With<Die>>,
    mut sub_spritesheet_query: Query<
        &mut SubSpritesheet,
        With<Die>>,
    mut movement_cooldown_query: Query<
        &mut MovementCooldown,
        With<Die>>,
) {
    let mut transform = transform_query.single_mut();
    let mut sprite = sprite_query.single_mut();
    let mut sub_spritesheet = sub_spritesheet_query.single_mut();
    let mut movement_cooldown = movement_cooldown_query.single_mut();
    
    movement_cooldown.tick(time.delta());
    if movement_cooldown.finished() {
        let mut direction = Vec3::splat(0.0);

        if keyboard_input.pressed(KeyCode::Left) { direction[0] -= 1.0; }
        if keyboard_input.pressed(KeyCode::Right) { direction[0] += 1.0; }
        if keyboard_input.pressed(KeyCode::Down) { direction[1] -= 1.0;}
        if keyboard_input.pressed(KeyCode::Up) { direction[1] += 1.0; }
    
        transform.translation = transform.translation + (direction * Vec3::splat(super::GRID_SIZE));
    
        if direction != Vec3::splat(0.0) {
            sprite.index = sub_spritesheet.next_sprite_index();
            movement_cooldown.reset();
        }
    }
}
