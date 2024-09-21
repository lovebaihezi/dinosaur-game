use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::{default, Camera2dBundle, Commands, Query, Transform},
    sprite::{Sprite, SpriteBundle},
    window::Window,
};

pub fn setup_ground(mut commands: Commands, window: Query<&Window>) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());

    // the ground width is the same as the window width
    // the ground height is 100 pixels
    // the ground x at 0, y at center of the window
    let window = window.iter().next().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.95, 0.95, 0.95),
            custom_size: Some(Vec2::new(window_width, 1.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, -window_height / 2.0, 0.0)),
        ..default()
    },));
}
