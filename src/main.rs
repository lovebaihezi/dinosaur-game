use bevy::prelude::*;
use dinosaur::{setup_dino, setup_ground};

fn main() {
    let exit = App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_ground, setup_dino))
        .run();
    match exit {
        AppExit::Success => {}
        AppExit::Error(_) => panic!("An error occurred while running the app"),
    }
}
