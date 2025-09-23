use bevy::{
    app::{FixedUpdate, Plugin},
    color::Color,
    math::{Vec2, Vec3},
    prelude::{default, Commands, Query, Res, Transform, With},
    sprite::Sprite,
    state::state::{OnEnter, OnTransition},
};

use crate::{components::Ground, GameScreen, GameStatus};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameScreen::PlayScreen), setup_ground)
            .add_systems(FixedUpdate, update_ground)
            .add_systems(OnEnter(GameScreen::ExitScreen), cleanup_ground);
    }
}

pub fn setup_ground(mut commands: Commands, game_status: Res<GameStatus>) {
    // the ground width is the same as the window width
    // the ground height is 100 pixels
    // the ground x at 0, y at center of the window
    let window_width = game_status.window_width;

    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.95),
            custom_size: Some(Vec2::new(window_width * 0.8, 1.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Ground,
    ));
}

/// Update the ground width, position on window resize
pub fn update_ground(
    game_status: Res<GameStatus>,
    mut query: Query<(&mut Transform, &Sprite), With<Ground>>,
) {
    let window_width = game_status.window_width;
    for (mut transform, sprite) in query.iter_mut() {
        let sprite_width = sprite.custom_size.unwrap().x;
        transform.scale = Vec3::new(window_width * 0.8 / sprite_width, 1.0, 1.0);
    }
}
