use bevy::prelude::{Camera2d, Commands};
use log::debug;

pub fn setup_2d_camera(mut commands: Commands) {
    debug!("Setup normal camera");
    commands.spawn(Camera2d);
}
