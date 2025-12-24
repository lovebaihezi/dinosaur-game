use std::time::Duration;

use crate::{
    game_logic::GameLogicPlugin, setup_2d_camera, update_window_size, DebugPlugin, DinoPlugin,
    GameControlPlugin, GameOverPlugin, GameScreen, GameStartPlugin, GameStatus, GroundPlugin,
    SpeedControlInfo, TreePlugin,
};
use bevy::{
    app::PluginGroupBuilder,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    prelude::*,
    text::{FontSmoothing, LineHeight},
    winit::WinitPlugin,
};
use bevy_kira_audio::prelude::AudioPlugin as KiraAudioPlugin;

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
                line_height: LineHeight::Px(16.0),
                font: default(),
                font_smoothing: FontSmoothing::default(),
            },
            text_color: Color::BLACK,
            enabled: true,
            refresh_interval: Duration::from_millis(166),
            // Frame time graph requires vertex storage features (supported by WebGPU)
            frame_time_graph_config: FrameTimeGraphConfig {
                enabled: true,
                min_fps: 60.,
                target_fps: 144.,
            },
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
            .init_state::<GameScreen>()
            .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
            .insert_resource(SpeedControlInfo {
                speed_increment: 100,
                max_game_speed: u64::MAX,
            })
            .add_plugins((
                DinoPlugin,
                GameControlPlugin,
                GameLogicPlugin,
                TreePlugin,
                GroundPlugin,
                GameStartPlugin,
                DebugPlugin,
                GameOverPlugin,
                KiraAudioPlugin,
            ));
        match app_type {
            AppType::Normal => {
                game.app
                    .add_systems(Startup, setup_2d_camera)
                    .add_systems(Update, update_window_size);
            }
            AppType::RenderToImageTesting => {
                todo!("Follow bevy render test example to setup one render to image test");
            }
        };
        game
    }

    pub fn run(mut self) -> AppExit {
        self.app.run()
    }
}
