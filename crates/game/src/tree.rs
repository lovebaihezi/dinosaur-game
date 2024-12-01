use bevy::{
    color::Color, math::{Vec2, Vec3}, prelude::{default, Commands, Query, Res, ResMut, Transform}, sprite::{Sprite, SpriteBundle}, time::{Time, Virtual}, ui::update, window::Window
};

use crate::{
    components::{Tree, MAX_GAME_SPEED, TREE_WIDTH, SPEED_INCREMENT},
    GameStatus,
};

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
    mut status: ResMut<GameStatus>,
) {
    if time.is_paused() {
        return;
    }
    let window = window.single();
    let window_width = window.width();
    for (mut transform, _) in tree_query.iter_mut() {
        transform.translation.x = if transform.translation.x < -window_width * 0.8 / 2.0 {
            update_game_speed(&mut status);
            window_width * 0.8 / 2.0
        } else {
            let more_hard_speed = (status.speed as f32).log2();
            transform.translation.x
                - time.delta_seconds() * (window_width / 3.0 + (TREE_WIDTH / 2.0) * more_hard_speed)
        };
    }
}


fn update_game_speed(status: &mut GameStatus) {
    unsafe {
        if status.speed < MAX_GAME_SPEED {
            let new_speed = status.speed.saturating_add(SPEED_INCREMENT);
            SPEED_INCREMENT = SPEED_INCREMENT.saturating_add(SPEED_INCREMENT);
            MAX_GAME_SPEED = MAX_GAME_SPEED.saturating_sub(SPEED_INCREMENT);
            status.speed = if new_speed >= MAX_GAME_SPEED {
                MAX_GAME_SPEED
            } else {
                new_speed
            };
        }
        println!("tree_move_animation Modified speed: {}  MAX_GAME_SPEED {}", status.speed, MAX_GAME_SPEED);
    }
}