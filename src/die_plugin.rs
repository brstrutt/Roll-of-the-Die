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
                    .with_system(move_die),
            );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let texture_handle = asset_server.load("resources/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(11.0, 11.0), 7, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(DieBundle::new(texture_atlas_handle));
}

const DIE_STARTING_POSITION: Vec3 = const_vec3!([0.0, 0.0, 1.0]);
const DIE_COLOR: Color = Color::rgb(0.5, 0.1, 0.1);
const DIE_SPEED: f32 = 100.0;

#[derive(Component)]
struct Die {
    spritesheet_indices: Vec<usize>,
}


#[derive(Bundle)]
struct DieBundle {
    die: Die,
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

impl DieBundle {
    fn new(texture_atlas_handle: Handle<TextureAtlas>) -> DieBundle {
        let spritesheet_indices: Vec<usize> = vec![1,2,3,4,5,6];
        let initial_index = spritesheet_indices[0];
        DieBundle { 
            die: Die { spritesheet_indices },
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
        }
    }
}

fn move_die(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Die>>,
) {
    let mut die_transform = query.single_mut();
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

    die_transform.translation = die_transform.translation + (direction * Vec3::splat(DIE_SPEED) * Vec3::splat(super::TIME_STEP));
}