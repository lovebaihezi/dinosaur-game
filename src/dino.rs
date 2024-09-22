use bevy::{
    color::Color,
    input::ButtonInput,
    math::{Vec2, Vec3},
    prelude::{default, Commands, KeyCode, MouseButton, Query, Res, Transform},
    sprite::{Sprite, SpriteBundle},
    window::Window,
};

const DINO_SIZE: Vec2 = Vec2::new(20.0, 20.0 / 0.618);

pub fn setup_dino(mut commands: Commands, window: Query<&Window>) {
    let window = window.iter().next().unwrap();
    let window_height = window.height();

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.95, 0.95, 0.95),
            custom_size: Some(DINO_SIZE),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, -window_height / 2.0, 0.0)),
        ..default()
    },));
}

/// Dino will jump when user press space, w, Up, k, or left mouse button
pub fn dino_jump_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &Sprite)>,
    window: Query<&Window>,
) {
    if keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::KeyW)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::KeyK)
        || mouse.just_pressed(MouseButton::Left)
    {
        let window = window.iter().next().unwrap();
        let window_height = window.height();

        for (mut transform, sprite) in query.iter_mut() {
            if transform.translation.y == -window_height / 2.0 {
                transform.translation.y = -window_height / 2.0 + 1.0;
            }
        }
    }
}
