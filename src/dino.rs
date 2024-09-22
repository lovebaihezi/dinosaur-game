use bevy::{
    color::Color,
    input::ButtonInput,
    log::info,
    math::{Vec2, Vec3},
    prelude::{default, Commands, KeyCode, Query, Res, Transform},
    sprite::{Sprite, SpriteBundle},
    window::Window,
};

const DINO_WIDTH: f32 = 20.0;
const DINO_SIZE: Vec2 = Vec2::new(DINO_WIDTH, DINO_WIDTH / 0.618);

pub fn setup_dino(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.05, 0.05, 0.05),
            custom_size: Some(DINO_SIZE),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
        ..default()
    },));
}

/// Dino will jump when user press space, w, Up, k, or left mouse button
pub fn dino_jump_system(
    mut query: Query<(&mut Transform, &Sprite)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    window: Query<&Window>,
) {
    if keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::KeyW)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::KeyK)
    {
        let window = window.single();
        let window_height = window.height();
        info!("Jump: {} {}", window.width(), window_height);

        for (mut transform, _sprite) in query.iter_mut() {
            if transform.translation.y == -window_height / 2.0 {
                transform.translation.y = -window_height / 2.0 + 1.0;
            }
        }
    }
}
