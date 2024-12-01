use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use dinosaur_game::{
    dino_jump_animation, dino_jump_system, dino_pos_fix_system, game_info,
    game_logic::{dino_touched_tree, reset_game},
    setup_camera, setup_dino, setup_game_control, setup_ground, setup_tree, tree_move_animation,
    update_ground, user_control, GameStatus, SpeedControlInfo,
};

fn main() {
    let default_plugin = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Dinosaur Game".to_string(),
            canvas: Some("#game".to_string()),
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        ..Default::default()
    });
    let exit = App::new()
        .add_plugins((default_plugin, FrameTimeDiagnosticsPlugin))
        .insert_resource(GameStatus { speed: 5, score: 0 })
        .insert_resource(SpeedControlInfo {
            speed_increment: 100,
            max_game_speed: u64::MAX,
        })
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
                (user_control, game_info).chain(),
                (dino_pos_fix_system, dino_jump_animation).chain(),
                tree_move_animation,
                (dino_touched_tree, reset_game).chain(),
            ),
        )
        .run();
    match exit {
        AppExit::Success => {}
        AppExit::Error(_) => panic!("An error occurred while running the app"),
    }
}
