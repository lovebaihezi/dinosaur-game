use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::Component,
    sprite::Sprite,
    time::{Time, Virtual},
    transform::components::Transform,
    utils::default,
};

#[derive(Component, Default)]
pub struct Dino {
    pub in_air_start_time: Option<Time<Virtual>>,
}

impl Dino {
    pub const WIDTH: f32 = 50.0;
    pub const HEIGHT: f32 = Self::WIDTH / 0.618;
    pub const SIZE: Vec2 = Vec2::new(Self::WIDTH, Self::WIDTH / 0.618);
    pub const JUMP_HIGH: f32 = Self::WIDTH / 0.618 * 2.4;

    pub fn new() -> (Self, Sprite, Transform) {
        (
            Self {
                in_air_start_time: None,
            },
            Sprite {
                color: Color::srgb(0.05, 0.05, 0.05),
                custom_size: Some(Self::SIZE),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, Self::WIDTH / 2.0 / 0.618, 0.0)),
        )
    }
}
