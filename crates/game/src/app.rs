use std::time::Duration;

use crate::{
    dino_jump_animation, dino_jump_system, dino_pos_fix_system, game_info,
    game_logic::{dino_touched_tree, reset_game},
    normal_app_setup, setup_dino, setup_game_control, setup_ground, setup_tree,
    test_functions::{render_to_image_setup, CaptureFramePlugin, ImageCopyPlugin, SceneController},
    tree_move_animation, update_ground, update_window_size, user_control, GameStatus,
    SpeedControlInfo,
};
use bevy::{
    app::{PluginGroupBuilder, ScheduleRunnerPlugin},
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
    RenderToImageTesting,
}

fn default_plugins(app_type: AppType) -> PluginGroupBuilder {
    let primary_window = match app_type {
        AppType::Normal => Some(Window {
            title: "Dinosaur Game".to_string(),
            canvas: Some("#game".to_string()),
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        AppType::RenderToImageTesting => None,
    };
    let plugin = DefaultPlugins.set(WindowPlugin {
        primary_window,
        ..Default::default()
    });
    match app_type {
        AppType::RenderToImageTesting => plugin
            .disable::<WinitPlugin>()
            .set(ImagePlugin::default_nearest()),
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
            .insert_resource(GameStatus {
                speed: 5,
                score: 0,
                window_width: 1920.0,
                window_height: 1080.0,
            })
            .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
            .insert_resource(SpeedControlInfo {
                speed_increment: 100,
                max_game_speed: u64::MAX,
            })
            .add_systems(
                Startup,
                (setup_ground, setup_dino, setup_tree, setup_game_control),
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
            AppType::Normal => {
                game.app
                    .add_systems(Startup, normal_app_setup)
                    .add_systems(Update, update_window_size);
            }
            AppType::RenderToImageTesting => {
                game.app
                    .add_systems(Startup, render_to_image_setup)
                    .add_plugins(ImageCopyPlugin)
                    .add_plugins(CaptureFramePlugin)
                    .add_plugins(ScheduleRunnerPlugin::run_loop(
                        // Run 60 times per second.
                        Duration::from_secs_f64(1.0 / 60.0),
                    ))
                    .init_resource::<SceneController>();
            }
        };
        game
    }

    pub fn run(mut self) -> AppExit {
        self.app.run()
    }
}
