use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::{default, Commands, Query, Res, Transform, With},
    sprite::{Sprite, SpriteBundle},
    time::{Time, Virtual},
    window::Window,
};

use crate::components::{Dino, Tree, TREE_WIDTH};

pub fn setup_tree(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let window_width = window.width();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.35, 0.35, 0.35),
                custom_size: Some(Vec2::new(TREE_WIDTH, TREE_WIDTH / 0.618)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                window_width - TREE_WIDTH,
                TREE_WIDTH / 2.0 / 0.618,
                0.0,
            )),
            ..default()
        },
        Tree::default(),
    ));
}

pub fn tree_move_animation(
    mut tree_query: Query<(&mut Transform, &mut Tree)>,
    time: Res<Time<Virtual>>,
    window: Query<&Window>,
) {
    if time.is_paused() {
        return;
    }
    let window = window.single();
    let window_width = window.width();
    for (mut transform, mut tree) in tree_query.iter_mut() {
        transform.translation.x = if transform.translation.x < -window_width * 0.8 / 2.0 {
            tree.dino_passed();
            window_width * 0.8 / 2.0
        } else {
            transform.translation.x
                - time.delta_seconds()
                    * (window_width / 3.0 + (TREE_WIDTH / 2.0) * tree.speed() as f32)
        };
    }
}
