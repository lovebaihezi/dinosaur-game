use bevy::{
    app::{FixedUpdate, Plugin, Startup},
    dev_tools::fps_overlay::FpsOverlayConfig,
    input::ButtonInput,
    prelude::{Commands, KeyCode, MouseButton, Query, Res, ResMut, Touches},
    state::state::{NextState, State},
    time::{Time, Virtual},
    window::Window,
};

use crate::GameScreen;

pub struct GameControlPlugin;

impl Plugin for GameControlPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(FixedUpdate, (show_fps_overlay, screen_changes));
    }
}

pub fn show_fps_overlay(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::F1) {
        overlay.enabled = !overlay.enabled;
    }
}

pub fn screen_changes(
    mut time: ResMut<Time<Virtual>>,
    window: Query<&Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    cur_screen: Res<State<GameScreen>>,
    mut next_screen: ResMut<NextState<GameScreen>>,
) {
    if let Ok(window) = window.single() {
        // If user back to window
        if window.focused
            && (keyboard.just_pressed(KeyCode::Space)
                || touches.any_just_pressed()
                || mouse.just_pressed(MouseButton::Left))
        {
            time.unpause();
        } else if !window.focused && !time.is_paused() {
            time.pause();
        };
    }
}
