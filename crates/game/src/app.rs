use std::time::Duration;

use crate::{
    game_logic::GameLogicPlugin, normal_app_setup, update_window_size, DinoPlugin,
    GameControlPlugin, GameScreen, GameStatus, SpeedControlInfo, TreePlugin,
};
use bevy::{
    app::PluginGroupBuilder,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::{FontSmoothing, LineHeight},
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
                line_height: LineHeight::Px(16.0),
                font: default(),
                font_smoothing: FontSmoothing::default(),
            },
            // We can also change color of the overlay
            text_color: Color::linear_rgba(0.0, 1.0, 0.0, 1.0),
            enabled: true,
            refresh_interval: Duration::from_millis(100),
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
            .add_plugins((DinoPlugin, GameControlPlugin, GameLogicPlugin, TreePlugin));
        match app_type {
            AppType::Normal => {
                game.app
                    .add_systems(Startup, normal_app_setup)
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
