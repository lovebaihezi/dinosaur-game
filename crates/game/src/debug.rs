use bevy::{
    app::{Plugin, Update},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::ButtonInput,
    prelude::{KeyCode, Res, ResMut, Resource},
    state::state::{NextState, State},
    time::{Time, Virtual},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

use crate::GameScreen;

/// Bevy version string (hardcoded since bevy doesn't expose VERSION constant)
const BEVY_VERSION: &str = "0.17";

/// Update interval for performance display in seconds (166ms = ~6 updates per second)
const PERF_DISPLAY_UPDATE_INTERVAL: f32 = 0.166;

/// Resource to track debug window state and cached performance values
#[derive(Resource)]
pub struct DebugWindowState {
    pub visible: bool,
    /// Cached FPS value for display
    cached_fps: Option<f64>,
    /// Cached frame time value for display (in seconds)
    cached_frame_time: Option<f64>,
    /// Cached frame count for display
    cached_frame_count: Option<u64>,
    /// Time since last performance update
    time_since_update: f32,
}

impl Default for DebugWindowState {
    fn default() -> Self {
        Self {
            visible: false,
            cached_fps: None,
            cached_frame_time: None,
            cached_frame_count: None,
            time_since_update: 0.0,
        }
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .init_resource::<DebugWindowState>()
            .add_systems(Update, toggle_debug_window)
            // Use EguiPrimaryContextPass for proper multi-pass mode input handling
            .add_systems(EguiPrimaryContextPass, show_debug_window);
    }
}

fn toggle_debug_window(input: Res<ButtonInput<KeyCode>>, mut state: ResMut<DebugWindowState>) {
    if input.just_pressed(KeyCode::F1) {
        state.visible = !state.visible;
    }
}

fn show_debug_window(
    mut contexts: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
    mut state: ResMut<DebugWindowState>,
    time: Res<Time>,
    mut virtual_time: ResMut<Time<Virtual>>,
    cur_screen: Res<State<GameScreen>>,
    mut next_screen: ResMut<NextState<GameScreen>>,
) {
    if !state.visible {
        return;
    }

    // Update cached values every PERF_DISPLAY_UPDATE_INTERVAL seconds
    state.time_since_update += time.delta_secs();
    if state.time_since_update >= PERF_DISPLAY_UPDATE_INTERVAL {
        state.time_since_update = 0.0;

        // Update FPS
        if let Some(fps) = diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
            state.cached_fps = fps.smoothed();
        }

        // Update frame time
        if let Some(frame_time) =
            diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            state.cached_frame_time = frame_time.smoothed();
        }

        // Update frame count
        if let Some(frame_count) =
            diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FRAME_COUNT)
        {
            state.cached_frame_count = frame_count.value().map(|v| v as u64);
        }
    }

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    egui::Window::new("Performance & Info")
        .default_pos([10.0, 10.0])
        .resizable(true)
        .collapsible(true)
        .movable(true)
        .show(ctx, |ui| {
            ui.heading("Performance");
            ui.separator();

            // FPS information
            if let Some(fps) = state.cached_fps {
                ui.label(format!("FPS: {:.1}", fps));
            } else {
                ui.label("FPS: N/A");
            }

            // Frame time (time per frame in milliseconds)
            // Frame time measures how long each frame takes to render
            if let Some(frame_time) = state.cached_frame_time {
                ui.label(format!("Frame Time: {:.2} ms", frame_time * 1000.0));
            } else {
                ui.label("Frame Time: N/A");
            }

            // Frame count
            if let Some(frame_count) = state.cached_frame_count {
                ui.label(format!("Frame Count: {}", frame_count));
            }

            ui.separator();
            ui.heading("Game State Control");
            ui.separator();

            let current_state = *cur_screen.get();
            ui.label(format!("Current State: {:?}", current_state));

            let is_paused = virtual_time.is_paused();
            ui.label(format!("Game Paused: {}", is_paused));

            ui.horizontal(|ui| {
                if ui
                    .button(if is_paused { "Resume" } else { "Pause" })
                    .clicked()
                {
                    if is_paused {
                        virtual_time.unpause();
                    } else {
                        virtual_time.pause();
                    }
                }

                if ui.button("Restart").clicked() {
                    virtual_time.unpause();
                    next_screen.set(GameScreen::StartScreen);
                }

                if ui.button("Play").clicked() {
                    virtual_time.unpause();
                    next_screen.set(GameScreen::PlayScreen);
                }
            });

            ui.separator();
            ui.heading("Version Info");
            ui.separator();

            ui.label(format!("Game Version: {}", env!("CARGO_PKG_VERSION")));
            ui.label(format!("Bevy Version: {}", BEVY_VERSION));

            ui.separator();
            ui.label("Press F1 to toggle this window");
        });
}
