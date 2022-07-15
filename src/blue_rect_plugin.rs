use bevy::prelude::*;

pub struct BlueRectPlugin;

impl Plugin for BlueRectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(draw_blue_rectangle);
    }
}

fn draw_blue_rectangle(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        ..default()
    });
}