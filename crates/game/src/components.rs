use std::ops::Mul;

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
    pub sprite: Sprite,
    pub transform: Transform,
    pub in_air_start_time: Option<Time<Virtual>>,
}

impl Dino {
    pub const WIDTH: f32 = 50.0;
    pub const HEIGHT: f32 = Self::WIDTH / 0.618;
    pub const SIZE: Vec2 = Vec2::new(Self::WIDTH, Self::WIDTH / 0.618);
    pub const JUMP_HIGH: f32 = Self::WIDTH / 0.618 * 2.4;

    pub fn new() -> Self {
        Self {
            sprite: Sprite {
                color: Color::srgb(0.05, 0.05, 0.05),
                custom_size: Some(Self::SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, Self::WIDTH / 2.0 / 0.618, 0.0)),
            in_air_start_time: None,
        }
    }
}

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

#[derive(Component)]
pub enum GameControl {
    Score,
}

#[derive(Component, Default)]
pub struct Tree {
    pub sprite: Sprite,
    pub transform: Transform,
}

impl Tree {
    pub const WIDTH: f32 = 30.0;
    pub fn new(original_pos: Vec3) -> Self {
        Self {
            sprite: Sprite {
                color: Color::srgb(0.35, 0.35, 0.35),
                custom_size: Some(Vec2::new(Tree::WIDTH, Tree::WIDTH / 0.618)),
                ..default()
            },
            transform: Transform::from_translation(original_pos),
        }
    }
}
