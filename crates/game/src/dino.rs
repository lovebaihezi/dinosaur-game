use bevy::{
    color::Color,
    input::ButtonInput,
    math::Vec3,
    prelude::{default, Commands, KeyCode, MouseButton, Query, Res, Touches, Transform, With},
    sprite::Sprite,
    time::{Time, Virtual},
};

use crate::{
    components::{Dino, DINO_SIZE, DINO_WIDTH, JUMP_HIGH},
    GameStatus,
};

pub fn setup_dino(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.05, 0.05, 0.05),
            custom_size: Some(DINO_SIZE),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, DINO_WIDTH / 2.0 / 0.618, 0.0)),
        Dino::default(),
    ));
}

pub fn dino_pos_fix_system(
    mut query: Query<(&mut Transform, &Sprite), With<Dino>>,
    game_status: Res<GameStatus>,
) {
    for (mut transform, _sprite) in query.iter_mut() {
        let window_width = game_status.window_width;
        transform.translation.x = -window_width / 2.0 + DINO_WIDTH / 2.0 + 0.2 * window_width;
    }
}

/// Dino will jump when user press space, w, Up, k, or left mouse button
pub fn dino_jump_system(
    mut dino_query: Query<&mut Dino>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touch: Res<Touches>,
    time: Res<Time<Virtual>>,
) {
    if time.is_paused() {
        return;
    }
    if keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::KeyW)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::KeyK)
        || mouse.just_pressed(MouseButton::Left)
        || touch.any_just_pressed()
    {
        for mut dino in dino_query.iter_mut() {
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
    if time.is_paused() {
        return;
    }
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
