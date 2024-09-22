use bevy::{
    color::Color,
    log::info,
    math::{Vec2, Vec3},
    prelude::{default, Camera2dBundle, Commands, Component, EventReader, Query, Transform, With},
    sprite::{Sprite, SpriteBundle},
    window::{Window, WindowResized},
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

    info!("Sprite init at {} {} {}", 0.0, -window_height / 2.0, 0.0);

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(1.0, 0.0, 0.0, 0.9),
            custom_size: Some(Vec2::new(window_width, 1.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, -window_height / 2.0, 0.0)),
        ..default()
    },));
}

/// Update the ground width, position on window resize
pub fn update_ground(
    mut query: Query<(&mut Transform, &Sprite)>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for resize_event in resize_reader.read() {
        for (mut transform, sprite) in query.iter_mut() {
            let sprite_width = sprite.custom_size.unwrap().x;
            transform.scale.x = resize_event.width / sprite_width;
            transform.translation.x = 0.0;
            transform.translation.y = -resize_event.height / 2.0;
            info!(
                "ground transform to {} {} {}",
                0.0,
                resize_event.height / 2.0,
                0.0
            );
        }
    }
}
