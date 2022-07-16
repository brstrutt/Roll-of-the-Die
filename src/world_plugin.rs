use bevy::prelude::*;
use crate::{GRID_SIZE, Collider, PressurePlate};


pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let spritesheet = crate::load_spritesheet(&asset_server, &mut texture_atlases);
    for x in 0 .. WORLD_SIZE {
        for y in 0 .. WORLD_SIZE {
            let tile_type = WORLD[x][y];

            let position = Vec3::new(
                (x as f32 - WORLD_CENTRE as f32)  * GRID_SIZE, 
                (y as f32 - WORLD_CENTRE as f32) * GRID_SIZE, 
                get_tile_height(&tile_type));

            let sprite_sheet_bundle = SpriteSheetBundle {
                texture_atlas: spritesheet.clone(),
                transform: Transform {
                    translation: position,
                    scale: Vec3::splat(super::PIXEL_SCALE),
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: get_sprite_index(&tile_type),
                    ..default()
                },
                ..default()
            };
            
            match tile_type {
                TileType::Floor => commands
                    .spawn()
                    .insert(Tile)
                    .insert_bundle(sprite_sheet_bundle),
                TileType::PressurePlate => commands
                    .spawn()
                    .insert(Tile)
                    .insert_bundle(sprite_sheet_bundle)
                    .insert(PressurePlate{ activated: false, number: 2}),
                TileType::Wall => commands
                    .spawn()
                    .insert(Tile)
                    .insert_bundle(sprite_sheet_bundle)
                    .insert(Collider),
            };
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Wall,
    Floor,
    PressurePlate
}

#[derive(Component)]
struct Tile;

fn get_sprite_index(tile_type: &TileType) -> usize {
    match tile_type {
        TileType::Floor => return 28,
        TileType::PressurePlate => return 30,
        TileType::Wall => return 47,
    }
}

fn get_tile_height(tile_type: &TileType) -> f32 {
    match tile_type {
        TileType::Floor => return 0.5,
        TileType::PressurePlate => return 0.5,
        TileType::Wall => return 1.0,
    }
}

const WORLD_SIZE: usize = 14;
const WORLD_CENTRE: usize = WORLD_SIZE / 2;

const WORLD: [[TileType; WORLD_SIZE]; WORLD_SIZE] = [
    [TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall],
    [TileType::Wall, TileType::PressurePlate, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::PressurePlate, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::PressurePlate, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::PressurePlate, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::PressurePlate, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::PressurePlate, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall],
];