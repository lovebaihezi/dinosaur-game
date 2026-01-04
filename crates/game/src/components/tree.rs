use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::Component,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

use crate::GameConfig;

#[derive(Component, Default)]
pub struct Tree;

impl Tree {
    /// Create a new Tree with the given config and position
    pub fn new(config: &GameConfig, original_pos: Vec3) -> (Self, Sprite, Transform) {
        (
            Self,
            Sprite {
                color: Color::srgb(0.35, 0.35, 0.35),
                custom_size: Some(Vec2::new(config.tree_width, config.tree_height)),
                ..default()
            },
            Transform::from_translation(original_pos),
        )
    }
}
