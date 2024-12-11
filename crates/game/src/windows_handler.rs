use bevy::{prelude::*, window::Window};

use crate::GameStatus;

pub fn update_window_size(window_query: Query<&Window>, mut game_status: ResMut<GameStatus>) {
    let window = window_query.single();
    let width = window.width();
    let height = window.height();
    game_status.window_width = width;
    game_status.window_height = height;
}
