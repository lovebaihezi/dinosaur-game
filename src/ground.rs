use bevy::{
    color::Color,
    log::info,
    math::{Vec2, Vec3},
    prelude::{default, Camera2dBundle, Commands, Component, EventReader, Query, Transform, With},
    sprite::{Sprite, SpriteBundle},
    window::{Window, WindowResized},
};

#[derive(Component)]
pub struct Ground;

pub fn setup_ground(mut commands: Commands, window: Query<&Window>) {
    // the ground width is the same as the window width
    // the ground height is 100 pixels
    // the ground x at 0, y at center of the window
    let window = window.iter().next().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    info!("Sprite init at {} {} {}", 0.0, -window_height / 2.0, 0.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.9),
                custom_size: Some(Vec2::new(window_width, 10.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Ground,
    ));
}

/// Update the ground width, position on window resize
pub fn update_ground(
    mut query: Query<(&mut Transform, &Sprite), With<Ground>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for resize_event in resize_reader.read() {
        for (mut transform, sprite) in query.iter_mut() {
            let sprite_width = sprite.custom_size.unwrap().x;
            transform.scale = Vec3::new(resize_event.width / sprite_width, 1.0, 1.0);
        }
    }
}
