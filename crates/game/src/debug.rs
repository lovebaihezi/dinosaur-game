use bevy::{
    app::{FixedUpdate, Plugin},
    dev_tools::fps_overlay::FpsOverlayConfig,
    input::ButtonInput,
    prelude::{KeyCode, Res, ResMut},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedUpdate, show_fps_overlay);
    }
}

fn show_fps_overlay(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::F1) {
        overlay.enabled = !overlay.enabled;
    }
}
