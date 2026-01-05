use bevy::prelude::Handle;
use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::Component,
    sprite::Sprite,
    time::{Time, Virtual},
    transform::components::Transform,
    utils::default,
};
use bevy_kira_audio::AudioInstance;

use crate::GameConfig;

/// Default dark color for dino
pub const DINO_DEFAULT_COLOR: Color = Color::srgb(0.05, 0.05, 0.05);
/// Red color for dino when touched
pub const DINO_TOUCHED_COLOR: Color = Color::srgb(0.8, 0.1, 0.1);

#[derive(Component, Default)]
pub struct Dino {
    pub in_air_start_time: Option<Time<Virtual>>,
    pub jump_sound: Option<Handle<AudioInstance>>,
    /// Whether the dino has been touched (turns red)
    pub is_touched: bool,
}

impl Dino {
    /// Create a new Dino with the given config
    pub fn new(config: &GameConfig) -> (Self, Sprite, Transform) {
        Self::new_with_color(config, DINO_DEFAULT_COLOR)
    }

    /// Create a new Dino with a specific color
    pub fn new_with_color(config: &GameConfig, color: Color) -> (Self, Sprite, Transform) {
        let size = Vec2::new(config.dino_width, config.dino_height);
        (
            Self {
                in_air_start_time: None,
                jump_sound: None,
                is_touched: false,
            },
            Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, config.dino_height / 2.0, 0.0)),
        )
    }
}
