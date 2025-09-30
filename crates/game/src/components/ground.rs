use core::ops::Mul;

use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::Component,
    sprite::Sprite,
    time::{Time, Virtual},
    transform::components::Transform,
    utils::default,
};

#[derive(Component)]
pub struct Ground {
    pub sprite: Sprite,
    pub transform: Transform,
}

impl Ground {
    pub fn new(width: impl Mul<f32, Output = f32>) -> Self {
        Self {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.95),
                custom_size: Some(Vec2::new(width * 0.8, 1.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        }
    }
}
