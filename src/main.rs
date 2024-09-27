use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use dinosaur::{
    dino_jump_animation, dino_jump_system, dino_pos_fix_system, fps_info,
    game_logic::dino_touched_tree, setup_camera, setup_dino, setup_game_control, setup_ground,
    setup_tree, tree_move_animation, update_ground, user_control,
};

fn main() {
    let exit = App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .add_systems(
            Startup,
            (
                setup_ground,
                setup_dino,
                setup_camera,
                setup_tree,
                setup_game_control,
            ),
        )
        .add_systems(
            Update,
            (
                update_ground,
                dino_jump_system,
                (user_control, fps_info).chain(),
                (dino_pos_fix_system, dino_jump_animation).chain(),
                tree_move_animation,
                dino_touched_tree,
            ),
        )
        .run();
    match exit {
        AppExit::Success => {}
        AppExit::Error(_) => panic!("An error occurred while running the app"),
    }
}
