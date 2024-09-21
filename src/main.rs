use bevy::prelude::*;
use dinosaur::{move_rectangle, setup};

fn main() {
    let exit = App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_rectangle)
        .run();
    match exit {
        AppExit::Success => {}
        AppExit::Error(_) => panic!("An error occurred while running the app"),
    }
}
