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
    pub const DINO_WIDTH: f32 = 50.0;
    pub const DINO_HEIGHT: f32 = Self::DINO_WIDTH / 0.618;
    pub const DINO_SIZE: Vec2 = Vec2::new(Self::DINO_WIDTH, Self::DINO_WIDTH / 0.618);
    pub const JUMP_HIGH: f32 = Self::DINO_WIDTH / 0.618 * 2.4;

    pub fn new() -> Self {
        Self {
            sprite: Sprite {
                color: Color::srgb(0.05, 0.05, 0.05),
                custom_size: Some(Self::DINO_SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.0,
                Self::DINO_WIDTH / 2.0 / 0.618,
                0.0,
            )),
            in_air_start_time: None,
        }
    }
}

#[derive(Component)]
pub enum GameControl {
    Score,
}

#[derive(Component)]
pub struct Ground;

#[derive(Component, Default)]
pub struct Tree {}

impl Tree {
    pub const TREE_WIDTH: f32 = 30.0;
    pub fn ready(&mut self) {}
    pub fn start(&mut self) {}
}
