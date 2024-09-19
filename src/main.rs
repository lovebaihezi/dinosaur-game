use bevy::prelude::*;

fn main() {
    let exit = App::new().add_plugins(DefaultPlugins {}).run();
    match exit {
        AppExit::Success => {}
        AppExit::Error(_) => panic!("An error occurred while running the app"),
    }
}
