use crate::{
    game_logic::GameLogicPlugin, setup_2d_camera, update_window_size, DebugPlugin, DinoPlugin,
    GameConfig, GameControlPlugin, GameOverPlugin, GameScreen, GameStartPlugin, GameStatus,
    GroundPlugin, SpeedControlInfo, TreePlugin,
};
use bevy::{app::PluginGroupBuilder, prelude::*, winit::WinitPlugin};
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
    let plugin = DefaultPlugins
        .set(WindowPlugin {
            primary_window,
            ..Default::default()
        })
        .set(AssetPlugin {
            #[cfg(target_arch = "wasm32")]
            meta_check: bevy::asset::AssetMetaCheck::Never,
            ..Default::default()
        });

    match app_type {
        AppType::RenderToImageTesting => plugin
            .disable::<WinitPlugin>()
            .set(ImagePlugin::default_nearest()),
        AppType::Normal => plugin,
    }
}

impl Game {
    pub fn init(app_type: AppType) -> Self {
        let mut game = Game { app: App::new() };
        // Load game config from file or use defaults
        let game_config = GameConfig::load_from_file();
        game.app
            .add_plugins(default_plugins(app_type))
            .insert_resource(GameStatus {
                speed: 5,
                score: 0,
                window_width: 1920.0,
                window_height: 1080.0,
            })
            .insert_resource(game_config)
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
