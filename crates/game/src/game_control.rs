use bevy::{
    app::{FixedUpdate, Plugin},
    input::ButtonInput,
    prelude::{KeyCode, MouseButton, Query, Res, ResMut, Touches},
    state::state::{NextState, State},
    time::{Time, Virtual},
    window::Window,
};
use bevy_egui::EguiContexts;

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
    mut contexts: EguiContexts,
) {
    // Check if egui wants pointer input (e.g., clicking on debug window)
    let egui_wants_pointer = contexts
        .ctx_mut()
        .map(|ctx| ctx.wants_pointer_input())
        .unwrap_or(false);

    // Only process mouse/touch if egui doesn't want the input
    let pointer_input = if egui_wants_pointer {
        false
    } else {
        touches.any_just_pressed() || mouse.just_pressed(MouseButton::Left)
    };

    if let Ok(window) = window.single() {
        if window.focused
            && (keyboard.just_pressed(KeyCode::Space) || pointer_input)
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
