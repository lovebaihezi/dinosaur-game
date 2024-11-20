use bevy::{app::ScheduleRunnerPlugin, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use dinosaur_game::{
    components::Dino,
    dino_jump_animation, dino_jump_system, dino_pos_fix_system, game_info,
    game_logic::{dino_touched_tree, reset_game},
    setup_camera, setup_dino, setup_game_control, setup_ground, setup_tree, tree_move_animation,
    update_ground, user_control, GameStatus,
};

#[test]
fn game_time_pause_as_no_focus() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(ScheduleRunnerPlugin::run_once()),
        FrameTimeDiagnosticsPlugin,
    ));

    app.insert_resource(GameStatus { speed: 5, score: 0 });
    app.insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)));
    app.add_systems(
        Startup,
        (
            setup_ground,
            setup_dino,
            setup_camera,
            setup_tree,
            setup_game_control,
        ),
    );
    app.add_systems(
        Update,
        (
            update_ground,
            dino_jump_system,
            (user_control, game_info).chain(),
            (dino_pos_fix_system, dino_jump_animation).chain(),
            tree_move_animation,
            (dino_touched_tree, reset_game).chain(),
        ),
    );

    // Setup test entities
    app.world_mut().spawn(Window::default());
    let dino_id = app.world_mut().spawn(Dino::default()).id();

    // Run systems
    app.run();

    // Check resulting changes
    assert!(app.world().get::<Dino>(dino_id).is_some());
    assert!(app
        .world()
        .get::<Dino>(dino_id)
        .unwrap()
        .in_air_start_time
        .is_none());
}
