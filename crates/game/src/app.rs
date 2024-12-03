use crate::{
    dino_jump_animation, dino_jump_system, dino_pos_fix_system, game_info,
    game_logic::{dino_touched_tree, reset_game},
    setup_camera, setup_dino, setup_game_control, setup_ground, setup_tree,
    test_functions::{CaptureFramePlugin, ImageCopyPlugin},
    tree_move_animation, update_ground, user_control, GameStatus, SpeedControlInfo,
};
use bevy::{
    app::PluginGroupBuilder,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
    winit::WinitPlugin,
};

pub struct Game {
    app: App,
}

/// # AppType: Control App init, plugins and systems
#[derive(Debug, Clone, Copy)]
pub enum AppType {
    Normal,
    TestNoRender,
}

fn default_plugins(app_type: AppType) -> PluginGroupBuilder {
    let primary_window = match app_type {
        AppType::Normal => Some(Window {
            title: "Dinosaur Game".to_string(),
            canvas: Some("#game".to_string()),
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        AppType::TestNoRender => None,
    };
    let plugin = DefaultPlugins.set(WindowPlugin {
        primary_window,
        ..Default::default()
    });
    match app_type {
        AppType::TestNoRender => plugin.disable::<WinitPlugin>(),
        AppType::Normal => plugin,
    }
}

fn fps_plugin() -> FpsOverlayPlugin {
    FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextFont {
                font_size: 16.0,
                font: default(),
                font_smoothing: FontSmoothing::default(),
            },
            // We can also change color of the overlay
            text_color: Color::linear_rgba(0.0, 1.0, 0.0, 1.0),
            enabled: true,
        },
    }
}

impl Game {
    pub fn init(app_type: AppType) -> Self {
        let mut game = Game { app: App::new() };
        game.app
            .add_plugins((default_plugins(app_type), fps_plugin()))
            .insert_resource(GameStatus { speed: 5, score: 0 })
            .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
            .insert_resource(SpeedControlInfo {
                speed_increment: 100,
                max_game_speed: u64::MAX,
            })
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
            );
        match app_type {
            AppType::Normal => {}
            AppType::TestNoRender => {
                game.app
                    .add_plugins(ImageCopyPlugin)
                    .add_plugins(CaptureFramePlugin);
            }
        };
        game
    }

    pub fn run(mut self) -> AppExit {
        self.app.run()
    }
}
