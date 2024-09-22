use bevy::prelude::*;
use dinosaur::{
    dino_jump_system, dino_pos_fix_system, setup_camera, setup_dino, setup_ground, update_ground,
};

fn main() {
    let exit = App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .add_systems(Startup, (setup_ground, setup_dino, setup_camera))
        .add_systems(
            Update,
            (update_ground, dino_jump_system, dino_pos_fix_system),
        )
        .run();
    match exit {
        AppExit::Success => {}
        AppExit::Error(_) => panic!("An error occurred while running the app"),
    }
}
