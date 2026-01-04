use core::ops::Mul;

use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::Component,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

use crate::GameConfig;

#[derive(Component)]
pub struct Ground;

impl Ground {
    pub fn new(config: &GameConfig, width: impl Mul<f32, Output = f32>) -> (Self, Sprite, Transform) {
        (
            Self,
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.95),
                custom_size: Some(Vec2::new(width * 0.8, 1.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, config.ground_y_pos, 0.0)),
        )
    }
}
