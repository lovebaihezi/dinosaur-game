use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::{default, Camera2dBundle, Commands, Query, Transform},
    sprite::{Sprite, SpriteBundle},
    window::Window,
};

pub fn setup_dino(mut commands: Commands, window: Query<&Window>) {
    let window = window.iter().next().unwrap();
    let window_height = window.height();

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.95, 0.95, 0.95),
            custom_size: Some(Vec2::new(10.0, 10.0 / 0.618)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, -window_height / 2.0, 0.0)),
        ..default()
    },));
}
