use bevy::{
    dev_tools::fps_overlay::FpsOverlayConfig,
    input::ButtonInput,
    prelude::{Commands, KeyCode, MouseButton, Query, Res, ResMut, Touches},
    time::{Time, Virtual},
    window::Window,
};

use crate::{components::Dino, GameStatus};

pub fn setup_game_control(commands: Commands, mut time: ResMut<Time<Virtual>>) {
    time.pause();
    _ = commands;
}

pub fn show_fps_overlay(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::F1) {
        overlay.enabled = !overlay.enabled;
    }
}

pub fn user_control(
    mut time: ResMut<Time<Virtual>>,
    mut dino_query: Query<&mut Dino>,
    window: Query<&Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
) {
    let window = window.single();
    for mut dino in dino_query.iter_mut() {
        if window.focused
            && time.is_paused()
            && (keyboard.just_pressed(KeyCode::Space)
                || touches.any_just_pressed()
                || mouse.just_pressed(MouseButton::Left))
        {
            dino.start();
            time.unpause();
        } else if !window.focused && !time.is_paused() {
            time.pause();
        };
    }
}

pub fn game_info(
    dino_query: Query<&Dino>,
    mut status: ResMut<GameStatus>,
    time: Res<Time<Virtual>>,
) {
    if !time.is_paused() {
        status.score += 1;
    }
    for dino in dino_query.iter() {
        if dino.is_over() {
            status.score = 0;
        }
    }
}
