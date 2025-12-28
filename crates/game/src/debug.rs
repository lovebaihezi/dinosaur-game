use bevy::{
    app::{Plugin, Update},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::ButtonInput,
    prelude::{KeyCode, Res, ResMut, Resource},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

/// Bevy version string (hardcoded since bevy doesn't expose VERSION constant)
const BEVY_VERSION: &str = "0.17";

/// Resource to track whether the debug window is visible
#[derive(Resource, Default)]
pub struct DebugWindowState {
    pub visible: bool,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .init_resource::<DebugWindowState>()
            .add_systems(Update, (toggle_debug_window, show_debug_window));
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
    state: Res<DebugWindowState>,
) {
    if !state.visible {
        return;
    }

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    egui::Window::new("Performance & Info")
        .default_pos([10.0, 10.0])
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Performance");
            ui.separator();

            // FPS information
            if let Some(fps) = diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    ui.label(format!("FPS: {:.1}", value));
                } else {
                    ui.label("FPS: N/A");
                }
            }

            // Frame time (time per frame)
            if let Some(frame_time) =
                diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FRAME_TIME)
            {
                if let Some(value) = frame_time.smoothed() {
                    ui.label(format!("Frame Time: {:.2} ms", value * 1000.0));
                } else {
                    ui.label("Frame Time: N/A");
                }
            }

            // Frame count
            if let Some(frame_count) =
                diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FRAME_COUNT)
            {
                if let Some(value) = frame_count.value() {
                    ui.label(format!("Frame Count: {}", value as u64));
                }
            }

            ui.separator();
            ui.heading("Version Info");
            ui.separator();

            ui.label(format!("Game Version: {}", env!("CARGO_PKG_VERSION")));
            ui.label(format!("Bevy Version: {}", BEVY_VERSION));

            ui.separator();
            ui.label("Press F1 to toggle this window");
        });
}
