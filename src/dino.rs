use bevy::{
    color::Color,
    input::ButtonInput,
    log::info,
    math::{Vec2, Vec3},
    prelude::{default, Commands, Component, EventReader, KeyCode, Query, Res, Transform, With},
    sprite::{Sprite, SpriteBundle},
    window::{Window, WindowResized},
};

#[derive(Component)]
pub struct Dino;

const DINO_WIDTH: f32 = 20.0;
const DINO_SIZE: Vec2 = Vec2::new(DINO_WIDTH, DINO_WIDTH / 0.618);

pub fn setup_dino(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let width = window.width();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.05, 0.05, 0.05),
                custom_size: Some(DINO_SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, DINO_WIDTH, 0.0)),
            ..default()
        },
        Dino,
    ));
}

pub fn dino_pos_fix_system(
    mut query: Query<(&mut Transform, &Sprite), With<Dino>>,
    mut events: EventReader<WindowResized>,
) {
    for e in events.read() {
        for (mut transform, _sprite) in query.iter_mut() {
            let window_width = e.width;
            transform.translation.x = -window_width / 2.0 + DINO_WIDTH / 2.0;
        }
    }
}

/// Dino will jump when user press space, w, Up, k, or left mouse button
pub fn dino_jump_system(
    mut query: Query<(&mut Transform, &Sprite), With<Dino>>,
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
