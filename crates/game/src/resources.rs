use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use serde::{Deserialize, Serialize};

/// Golden ratio constant used for proportional sizing
const GOLDEN_RATIO: f32 = 0.618;

#[derive(Debug, Default, Resource)]
pub struct GameStatus {
    pub score: u64,
    pub speed: u64,
    pub window_width: f32,
    pub window_height: f32,
}

#[derive(Debug, Default, Resource)]
pub struct SpeedControlInfo {
    pub speed_increment: u64,
    pub max_game_speed: u64,
}

#[derive(Resource, Deref)]
pub struct DinoJumpMusic(pub Handle<AudioSource>);

/// Configuration for game entities that can be modified via egui and exported/imported
#[derive(Debug, Clone, Resource, Serialize, Deserialize)]
pub struct GameConfig {
    /// Dino width in pixels
    pub dino_width: f32,
    /// Dino height in pixels
    pub dino_height: f32,
    /// Dino jump height in pixels
    pub dino_jump_height: f32,
    /// Dino X position offset (0.0 to 1.0, percentage of window width from left)
    pub dino_x_offset: f32,
    /// Tree width in pixels
    pub tree_width: f32,
    /// Tree height in pixels
    pub tree_height: f32,
    /// Ground Y position (0.0 is center of screen)
    pub ground_y_pos: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        // These match the original hardcoded constants
        let dino_width = 50.0;
        let tree_width = 30.0;
        Self {
            dino_width,
            dino_height: dino_width / GOLDEN_RATIO, // ~80.9
            dino_jump_height: dino_width / GOLDEN_RATIO * 2.4, // ~194.2
            dino_x_offset: 0.2,
            tree_width,
            tree_height: tree_width / GOLDEN_RATIO, // ~48.5
            ground_y_pos: 0.0,
        }
    }
}

impl GameConfig {
    /// Default config file name
    pub const CONFIG_FILE: &'static str = "game_config.json";

    /// Load config from a JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Export config to a JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Load config from file, returns default if file doesn't exist or is invalid
    pub fn load_from_file() -> Self {
        std::fs::read_to_string(Self::CONFIG_FILE)
            .ok()
            .and_then(|content| Self::from_json(&content).ok())
            .unwrap_or_default()
    }

    /// Save config to file
    pub fn save_to_file(&self) -> Result<(), std::io::Error> {
        let json = self
            .to_json()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        std::fs::write(Self::CONFIG_FILE, json)
    }
}
