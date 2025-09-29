use bevy::{
    app::{FixedUpdate, Plugin},
    math::Vec3,
    prelude::{Commands, Query, Res},
    state::state::OnEnter,
};

use crate::{components::Ground, utils::cleanup_component, GameScreen, GameStatus};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameScreen::PlayScreen), setup_ground)
            .add_systems(FixedUpdate, update_ground)
            .add_systems(OnEnter(GameScreen::ExitScreen), cleanup_component::<Ground>);
    }
}

pub fn setup_ground(mut commands: Commands, game_status: Res<GameStatus>) {
    // the ground width is the same as the window width
    // the ground height is 100 pixels
    // the ground x at 0, y at center of the window
    let window_width = game_status.window_width;

    commands.spawn(Ground::new(window_width));
}

/// Update the ground width, position on window resize
pub fn update_ground(game_status: Res<GameStatus>, mut query: Query<&mut Ground>) {
    let window_width = game_status.window_width;
    for mut ground in query.iter_mut() {
        let sprite_width = ground.sprite.custom_size.unwrap().x;
        ground.transform.scale = Vec3::new(window_width * 0.8 / sprite_width, 1.0, 1.0);
    }
}
