use bevy::prelude::*;
use crate::{GRID_SIZE, Collider, PressurePlate, Spritesheet};


pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, setup)
            .add_systems(Update, update_pressure_plate_appearence);
    }
}

fn setup(
    mut commands: Commands,
    spritesheet: Res<Spritesheet>,
) {
    let spritesheet = spritesheet.0.clone();
    for y in 0 .. WORLD_SIZE {
        for x in 0 .. WORLD_SIZE {
            let tile_type = WORLD[y][x];

            let position = Vec3::new(
                (x as f32 - WORLD_CENTRE as f32)  * GRID_SIZE, 
                (y as f32 - WORLD_CENTRE as f32) * GRID_SIZE * -1.0, 
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
                    .spawn((Tile, sprite_sheet_bundle)),
                TileType::PressurePlate1 => commands
                    .spawn((Tile, sprite_sheet_bundle, PressurePlate{ activated: false, number: 1})),
                TileType::PressurePlate2 => commands
                    .spawn((Tile, sprite_sheet_bundle, PressurePlate{ activated: false, number: 2})),
                TileType::PressurePlate3 => commands
                    .spawn((Tile, sprite_sheet_bundle, PressurePlate{ activated: false, number: 3})),
                TileType::PressurePlate4 => commands
                    .spawn((Tile, sprite_sheet_bundle, PressurePlate{ activated: false, number: 4})),
                TileType::PressurePlate5 => commands
                    .spawn((Tile, sprite_sheet_bundle, PressurePlate{ activated: false, number: 5})),
                TileType::PressurePlate6 => commands
                    .spawn((Tile, sprite_sheet_bundle, PressurePlate{ activated: false, number: 6})),
                TileType::Wall => commands
                    .spawn((Tile, sprite_sheet_bundle, Collider)),
            };
        }
    }
}

fn update_pressure_plate_appearence(
    mut pressure_plates_query: Query<(& PressurePlate, &mut TextureAtlasSprite)>,
) {
    for (pressure_plate, mut texture_atlas_sprite) in pressure_plates_query.iter_mut() {
        if pressure_plate.activated {
            texture_atlas_sprite.index = get_pressure_plate_face_sprite_index(pressure_plate.number) - 7;
        }
        else {
            texture_atlas_sprite.index = get_pressure_plate_face_sprite_index(pressure_plate.number);
        }
    }
}

fn get_pressure_plate_face_sprite_index(face_num: usize) -> usize {
    match face_num {
        1 => return 29,
        2 => return 30,
        3 => return 31,
        4 => return 32,
        5 => return 33,
        6 => return 34,
        _ => return 28,
    }
}

#[derive(Copy, Clone, PartialEq)]
enum TileType {
    Wall,
    Floor,
    PressurePlate1,
    PressurePlate2,
    PressurePlate3,
    PressurePlate4,
    PressurePlate5,
    PressurePlate6,
}

#[derive(Component)]
struct Tile;

fn get_sprite_index(tile_type: &TileType) -> usize {
    match tile_type {
        TileType::Floor => return 28,
        TileType::PressurePlate1 => return 29,
        TileType::PressurePlate2 => return 30,
        TileType::PressurePlate3 => return 31,
        TileType::PressurePlate4 => return 32,
        TileType::PressurePlate5 => return 33,
        TileType::PressurePlate6 => return 34,
        TileType::Wall => return 47,
    }
}

fn get_tile_height(tile_type: &TileType) -> f32 {
    match tile_type {
        TileType::Floor => return 0.5,
        TileType::PressurePlate1 | 
        TileType::PressurePlate2 | 
        TileType::PressurePlate3 | 
        TileType::PressurePlate4 | 
        TileType::PressurePlate5 | 
        TileType::PressurePlate6 => return 0.5,
        TileType::Wall => return 1.0,
    }
}

pub const WORLD_SIZE: usize = 14;
const WORLD_CENTRE: usize = WORLD_SIZE / 2;

const WORLD: [[TileType; WORLD_SIZE]; WORLD_SIZE] = [
    [TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall],
    [TileType::Wall, TileType::PressurePlate1, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::PressurePlate6, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::PressurePlate2, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::PressurePlate5, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall, TileType::Floor, TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::PressurePlate3, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall, TileType::Floor, TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall, TileType::PressurePlate2, TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall],
    [TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall, TileType::Floor, TileType::Wall, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::PressurePlate4, TileType::Wall],
    [TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall],
];