use bevy::{
    app::{Plugin, Update},
    ecs::query::With,
    math::Vec3,
    prelude::{Commands, Query, Res, ResMut},
    sprite::Sprite,
    state::state::{OnEnter, OnExit},
    time::{Time, Virtual},
    transform::components::Transform,
};

use crate::{
    components::Tree, utils::cleanup_component, GameConfig, GameScreen, GameStatus,
    SpeedControlInfo,
};

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameScreen::PlayScreen), setup_tree)
            .add_systems(
                Update,
                (tree_move_animation, update_tree_sprite_from_config),
            )
            .add_systems(OnExit(GameScreen::PlayScreen), cleanup_component::<Tree>);
    }
}

fn setup_tree(mut commands: Commands, status: Res<GameStatus>, config: Res<GameConfig>) {
    let window_width = status.window_width;
    let tree_pos = Vec3::new(
        window_width - config.tree_width,
        config.tree_height / 2.0,
        0.0,
    );

    commands.spawn(Tree::new(&config, tree_pos));
}

fn tree_move_animation(
    mut tree_query: Query<(&mut Transform, &Sprite), With<Tree>>,
    time: Res<Time<Virtual>>,
    mut status: ResMut<GameStatus>,
    mut speed_control_info: ResMut<SpeedControlInfo>,
    config: Res<GameConfig>,
) {
    if time.is_paused() {
        return;
    }
    let window_width = status.window_width;
    for (mut transform, sprite) in tree_query.iter_mut() {
        let tree_width = sprite.custom_size.map(|s| s.x).unwrap_or(config.tree_width);
        transform.translation.x = if transform.translation.x < -window_width * 0.8 / 2.0 {
            update_game_speed(&mut status, &mut speed_control_info);
            window_width * 0.8 / 2.0
        } else {
            let more_hard_speed = (status.speed as f32).log2();
            transform.translation.x
                - time.delta_secs() * (window_width / 3.0 + (tree_width / 2.0) * more_hard_speed)
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

/// Update tree sprite size based on config changes in real-time
fn update_tree_sprite_from_config(
    mut query: Query<(&mut Sprite, &mut Transform), With<Tree>>,
    config: Res<GameConfig>,
) {
    for (mut sprite, mut transform) in query.iter_mut() {
        let new_size = bevy::math::Vec2::new(config.tree_width, config.tree_height);
        if sprite.custom_size != Some(new_size) {
            sprite.custom_size = Some(new_size);
            // Update y position to keep tree on ground
            transform.translation.y = config.tree_height / 2.0;
        }
    }
}
