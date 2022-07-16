use bevy::{
    prelude::*,
    math::const_vec3,
    core::FixedTimestep,
};

pub struct DiePlugin;

impl Plugin for DiePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(super::TIME_STEP as f64))
                    .with_system(tick_movement_timer)
                    .with_system(move_die),
            );
    }
}

const DIE_STARTING_POSITION: Vec3 = const_vec3!([0.0, 0.0, 1.0]);
const DIE_EDGE_LEN_PIX: f32 = 11.0;
const DIE_EDGE_LEN: f32 = DIE_EDGE_LEN_PIX * super::PIXEL_SCALE;
const MOVEMENT_TICK: f32 = 0.5;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let texture_handle = asset_server.load("resources/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(DIE_EDGE_LEN_PIX), 7, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(DieBundle::new(texture_atlas_handle));
}

#[derive(Component, Deref, DerefMut)]
struct MovementTimer(Timer);

#[derive(Component)]
struct Die;

#[derive(Component)]
struct SubSpritesheet {
    spritesheet_indices: Vec<usize>,
    current_index: usize,
}


#[derive(Bundle)]
struct DieBundle {
    die: Die,
    movement_timer: MovementTimer,
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
            movement_timer: MovementTimer(Timer::from_seconds(MOVEMENT_TICK, true)),
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
    mut movement_timer_query: Query<
        &mut MovementTimer,
        With<Die>>,
) {
    let mut transform = transform_query.single_mut();
    let mut sprite = sprite_query.single_mut();
    let mut sub_spritesheet = sub_spritesheet_query.single_mut();
    let mut movement_timer = movement_timer_query.single_mut();
    
    if movement_timer.paused() {
        let mut direction = Vec3::splat(0.0);

        if keyboard_input.pressed(KeyCode::Left) {
            direction[0] -= 1.0;
        }
    
        if keyboard_input.pressed(KeyCode::Right) {
            direction[0] += 1.0;
        }
    
        if keyboard_input.pressed(KeyCode::Down) {
            direction[1] -= 1.0;
        }
    
        if keyboard_input.pressed(KeyCode::Up) {
            direction[1] += 1.0;
        }
    
        transform.translation = transform.translation + (direction * Vec3::splat(DIE_EDGE_LEN));
    
        if direction != Vec3::splat(0.0) {
            let next_sprite_index = (sub_spritesheet.current_index + 1) % sub_spritesheet.spritesheet_indices.len();
            sprite.index = sub_spritesheet.spritesheet_indices[next_sprite_index];
            sub_spritesheet.current_index = next_sprite_index;
            movement_timer.unpause();
        }
    }
}

fn tick_movement_timer(
    time: Res<Time>,
    mut movement_timer_query: Query<
        &mut MovementTimer,
        With<Die>>,
) {
    let mut movement_timer = movement_timer_query.single_mut();
    if !movement_timer.paused() {
        movement_timer.tick(time.delta());
        if movement_timer.just_finished() { 
            movement_timer.pause();
        }
    }
}