use bevy::prelude::*;

use crate::GameScreen;

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScreen::StartScreen), show_start_message);
    }
}

fn show_start_message() {
    println!("Game Start! Press Space or Click to Play!");
}
