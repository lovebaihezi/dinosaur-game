use bevy::{
    app::{Plugin, Update},
    ecs::query::With,
    math::Vec3,
    prelude::{Commands, Query, Res},
    sprite::Sprite,
    state::state::OnEnter,
    transform::components::Transform,
};

use crate::{components::Ground, utils::cleanup_component, GameConfig, GameScreen, GameStatus};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameScreen::PlayScreen), setup_ground)
            .add_systems(Update, update_ground)
            .add_systems(
                OnEnter(GameScreen::GameOverScreen),
                cleanup_component::<Ground>,
            );
    }
}

fn setup_ground(mut commands: Commands, game_status: Res<GameStatus>, config: Res<GameConfig>) {
    // the ground width is the same as the window width
    // the ground height is 100 pixels
    // the ground x at 0, y at center of the window
    let window_width = game_status.window_width;

    commands.spawn(Ground::new(&config, window_width));
}

/// Update the ground width, position on window resize
fn update_ground(
    game_status: Res<GameStatus>,
    mut query: Query<(&mut Transform, &Sprite), With<Ground>>,
) {
    let window_width = game_status.window_width;
    for (mut transform, sprite) in query.iter_mut() {
        let sprite_width = sprite.custom_size.unwrap().x;
        transform.scale = Vec3::new(window_width * 0.8 / sprite_width, 1.0, 1.0);
    }
}
