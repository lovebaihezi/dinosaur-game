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

#[derive(Component, Default)]
pub struct Dino {
    pub in_air_start_time: Option<Time<Virtual>>,
    pub jump_sound: Option<Handle<AudioInstance>>,
}

impl Dino {
    /// Create a new Dino with the given config
    pub fn new(config: &GameConfig) -> (Self, Sprite, Transform) {
        let size = Vec2::new(config.dino_width, config.dino_height);
        (
            Self {
                in_air_start_time: None,
                jump_sound: None,
            },
            Sprite {
                color: Color::srgb(0.05, 0.05, 0.05),
                custom_size: Some(size),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, config.dino_height / 2.0, 0.0)),
        )
    }
}
