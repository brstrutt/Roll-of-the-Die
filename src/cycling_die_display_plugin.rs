use bevy::prelude::*;

pub struct CyclingDieDisplayPlugin;

impl Plugin for CyclingDieDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(draw_die_face)
            .add_system(animate_sprite);
    }
}


#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = ((sprite.index) % 6) + 1;
        }
    }
}

fn draw_die_face(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let texture_handle = asset_server.load("resources/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(11.0, 11.0), 7, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(-20.0, -200.0, 0.0)),
        ..default()
    })
    .insert(AnimationTimer(Timer::from_seconds(0.5, true)));
}