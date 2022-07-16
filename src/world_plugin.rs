use bevy::prelude::*;
use crate::GRID_SIZE;


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
            let position = Vec3::new((x as f32 - WORLD_CENTRE as f32)  * GRID_SIZE, (y as f32 - WORLD_CENTRE as f32) * GRID_SIZE, 0.5);

            commands
                .spawn()
                .insert(Tile { type_of: TileType::Floor})
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: spritesheet.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(super::PIXEL_SCALE),
                        ..default()
                    },
                    sprite: TextureAtlasSprite {
                        index: 28,
                        ..default()
                    },
                    ..default()
                });
        }
    }
}

enum TileType {
    Wall,
    Floor,
}

#[derive(Component)]
struct Tile {
    type_of: TileType,
}

const WORLD_SIZE: usize = 14;
const WORLD_CENTRE: usize = WORLD_SIZE / 2;