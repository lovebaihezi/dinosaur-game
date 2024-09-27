use bevy::{
    color::Color,
    input::ButtonInput,
    math::{Vec2, Vec3},
    prelude::{
        default, Commands, EventReader, KeyCode, MouseButton, Query, Res, Touches, Transform, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Time, Virtual},
    window::WindowResized,
};

use crate::components::Dino;

const DINO_WIDTH: f32 = 50.0;
const DINO_SIZE: Vec2 = Vec2::new(DINO_WIDTH, DINO_WIDTH / 0.618);
const JUMP_HIGH: f32 = DINO_WIDTH / 0.618 * 1.5;

pub fn setup_dino(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.05, 0.05, 0.05),
                custom_size: Some(DINO_SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, DINO_WIDTH / 2.0 / 0.618, 0.0)),
            ..default()
        },
        Dino::default(),
    ));
}

pub fn dino_pos_fix_system(
    mut query: Query<(&mut Transform, &Sprite), With<Dino>>,
    mut events: EventReader<WindowResized>,
) {
    for e in events.read() {
        for (mut transform, _sprite) in query.iter_mut() {
            let window_width = e.width;
            transform.translation.x = -window_width / 2.0 + DINO_WIDTH / 2.0 + 0.1 * window_width;
        }
    }
}

/// Dino will jump when user press space, w, Up, k, or left mouse button
pub fn dino_jump_system(
    mut query: Query<&mut Dino>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touch: Res<Touches>,
    time: Res<Time<Virtual>>,
) {
    if keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::KeyW)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::KeyK)
        || mouse.just_pressed(MouseButton::Left)
        || touch.any_just_pressed()
    {
        for mut dino in query.iter_mut() {
            if dino.in_air_start_time.is_some() {
                continue;
            } else {
                dino.in_air_start_time = Some(*time);
            }
        }
    }
}

pub fn dino_jump_animation(
    time: Res<Time<Virtual>>,
    mut query: Query<(&mut Transform, &mut Dino)>,
) {
    for (mut transform, mut dino) in query.iter_mut() {
        if let Some(start_time) = dino.in_air_start_time {
            let elapsed = time.elapsed() - start_time.elapsed();
            // Over
            let y = if elapsed.as_millis() > 500 {
                dino.in_air_start_time = None;
                DINO_WIDTH / 2.0 / 0.618
            } else {
                let x = elapsed.as_millis() as f64 / 500.0 * std::f64::consts::PI;
                let x = x as f32;
                x.sin() * JUMP_HIGH + DINO_WIDTH / 2.0 / 0.618
            };
            transform.translation.y = y;
        }
    }
}
