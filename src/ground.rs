use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::{default, Camera2dBundle, Commands, Component, Query, Res, Transform},
    sprite::{Sprite, SpriteBundle},
    time::Time,
    window::Window,
};

#[derive(Component)]
pub struct Velocity(Vec2);

pub fn setup(mut commands: Commands, window: Query<&Window>) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());

    // the ground width is the same as the window width
    // the ground height is 100 pixels
    // the ground x at 0, y at center of the window
    let window = window.iter().next().unwrap();
    let window_width = window.width();
    let window_height = window.height();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.25, 0.75, 0.25),
                custom_size: Some(Vec2::new(window_width, 3.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -window_height / 2.0, 0.0)),
            ..default()
        },
        Velocity(Vec2::new(0.0, 0.0)),
    ));
}

pub fn move_rectangle(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_seconds();
    }
}
