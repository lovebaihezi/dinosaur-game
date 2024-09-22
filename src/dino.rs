use bevy::{
    color::Color,
    input::ButtonInput,
    log::info,
    math::{Vec2, Vec3},
    prelude::{default, Commands, Component, EventReader, KeyCode, Query, Res, Transform, With},
    sprite::{Sprite, SpriteBundle},
    time::{Time, Virtual},
    window::WindowResized,
};

#[derive(Component)]
pub struct Dino {
    in_air_start_time: Option<Time<Virtual>>,
}

const DINO_WIDTH: f32 = 20.0;
const DINO_SIZE: Vec2 = Vec2::new(DINO_WIDTH, DINO_WIDTH / 0.618);
const JUMP_HIGH: f32 = 100.0;

pub fn setup_dino(mut commands: Commands) {
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
        Dino {
            in_air_start_time: None,
        },
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
    mut query: Query<&mut Dino>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Virtual>>,
) {
    if keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::KeyW)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::KeyK)
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

pub fn dino_jump_animation(time: Res<Time>, mut query: Query<(&mut Transform, &mut Dino)>) {
    for (mut transform, mut dino) in query.iter_mut() {
        if let Some(start_time) = dino.in_air_start_time {
            let elapsed = time.elapsed() - start_time.elapsed();
            // Over
            let y = if elapsed.as_millis() > 500 {
                dino.in_air_start_time = None;
                DINO_WIDTH
            } else {
                let x = elapsed.as_millis() as f64 / 500.0 * std::f64::consts::PI;
                let x = x as f32;
                let y = x.sin() * JUMP_HIGH + DINO_WIDTH;
                y
            };
            transform.translation.y = y;
        }
    }
}
