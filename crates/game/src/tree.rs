use bevy::{
    app::{FixedUpdate, Plugin},
    ecs::query::With,
    math::Vec3,
    prelude::{Commands, Query, Res, ResMut},
    state::state::OnEnter,
    time::{Time, Virtual},
    transform::components::Transform,
};

use crate::{components::Tree, utils::cleanup_component, GameScreen, GameStatus, SpeedControlInfo};

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameScreen::PlayScreen), setup_tree)
            .add_systems(FixedUpdate, tree_move_animation)
            .add_systems(OnEnter(GameScreen::ExitScreen), cleanup_component::<Tree>);
    }
}

fn setup_tree(mut commands: Commands, status: Res<GameStatus>) {
    let window_width = status.window_width;
    let tree_pos = Vec3::new(window_width - Tree::WIDTH, Tree::WIDTH / 2.0 / 0.618, 0.0);

    commands.spawn(Tree::new(tree_pos));
}

fn tree_move_animation(
    mut tree_query: Query<&mut Transform, With<Tree>>,
    time: Res<Time<Virtual>>,
    mut status: ResMut<GameStatus>,
    mut speed_control_info: ResMut<SpeedControlInfo>,
) {
    if time.is_paused() {
        return;
    }
    let window_width = status.window_width;
    for mut transform in tree_query.iter_mut() {
        transform.translation.x = if transform.translation.x < -window_width * 0.8 / 2.0 {
            update_game_speed(&mut status, &mut speed_control_info);
            window_width * 0.8 / 2.0
        } else {
            let more_hard_speed = (status.speed as f32).log2();
            transform.translation.x
                - time.delta_secs() * (window_width / 3.0 + (Tree::WIDTH / 2.0) * more_hard_speed)
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
