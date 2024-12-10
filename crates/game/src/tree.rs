use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::{default, Commands, Query, Res, ResMut, Transform},
    sprite::Sprite,
    time::{Time, Virtual},
};

use crate::{
    components::{Tree, TREE_WIDTH},
    GameStatus, SpeedControlInfo,
};

pub fn setup_tree(mut commands: Commands, status: Res<GameStatus>) {
    let window_width = status.window_width;

    commands.spawn((
        Sprite {
            color: Color::srgb(0.35, 0.35, 0.35),
            custom_size: Some(Vec2::new(TREE_WIDTH, TREE_WIDTH / 0.618)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            window_width - TREE_WIDTH,
            TREE_WIDTH / 2.0 / 0.618,
            0.0,
        )),
        Tree::default(),
    ));
}

pub fn tree_move_animation(
    mut tree_query: Query<(&mut Transform, &mut Tree)>,
    time: Res<Time<Virtual>>,
    mut status: ResMut<GameStatus>,
    mut speed_control_info: ResMut<SpeedControlInfo>,
) {
    if time.is_paused() {
        return;
    }
    let window_width = status.window_width;
    for (mut transform, _) in tree_query.iter_mut() {
        transform.translation.x = if transform.translation.x < -window_width * 0.8 / 2.0 {
            update_game_speed(&mut status, &mut speed_control_info);
            window_width * 0.8 / 2.0
        } else {
            let more_hard_speed = (status.speed as f32).log2();
            transform.translation.x
                - time.delta_secs() * (window_width / 3.0 + (TREE_WIDTH / 2.0) * more_hard_speed)
        };
    }
}

fn update_game_speed(status: &mut GameStatus, info: &mut SpeedControlInfo) {
    if status.speed < info.max_game_speed {
        let new_speed = status.speed.saturating_add(info.speed_increment);
        info.speed_increment = info.speed_increment.saturating_add(info.speed_increment);
        info.max_game_speed = info.max_game_speed.saturating_sub(info.speed_increment);
        status.speed = if new_speed >= info.max_game_speed {
            info.max_game_speed
        } else {
            new_speed
        };
    }
}
