use bevy::{
    app::{FixedUpdate, Plugin},
    input::ButtonInput,
    prelude::{KeyCode, MouseButton, Query, Res, ResMut, Touches},
    state::state::{NextState, State},
    time::{Time, Virtual},
    window::Window,
};

use crate::GameScreen;

pub struct GameControlPlugin;

impl Plugin for GameControlPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(FixedUpdate, screen_changes);
    }
}

fn screen_changes(
    mut time: ResMut<Time<Virtual>>,
    window: Query<&Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    cur_screen: Res<State<GameScreen>>,
    mut next_screen: ResMut<NextState<GameScreen>>,
) {
    if let Ok(window) = window.single() {
        if window.focused
            && (keyboard.just_pressed(KeyCode::Space)
                || touches.any_just_pressed()
                || mouse.just_pressed(MouseButton::Left))
        {
            if *cur_screen == GameScreen::UnfocusedPauseScreen {
                time.unpause();
                next_screen.set(GameScreen::PlayScreen);
            }
        } else if !window.focused && !time.is_paused() && *cur_screen == GameScreen::PlayScreen {
            time.pause();
            next_screen.set(GameScreen::UnfocusedPauseScreen);
        };

        if window.focused && keyboard.just_released(KeyCode::Escape) {
            if *cur_screen == GameScreen::ManuallyPauseScreen {
                time.unpause();
                next_screen.set(GameScreen::PlayScreen);
            } else {
                time.pause();
                next_screen.set(GameScreen::ManuallyPauseScreen);
            }
        }
    }
}
