use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::Component,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

#[derive(Component, Default)]
pub struct Tree;

impl Tree {
    pub const WIDTH: f32 = 30.0;
    pub fn new(original_pos: Vec3) -> (Self, Sprite, Transform) {
        (
            Self,
            Sprite {
                color: Color::srgb(0.35, 0.35, 0.35),
                custom_size: Some(Vec2::new(Tree::WIDTH, Tree::WIDTH / 0.618)),
                ..default()
            },
            Transform::from_translation(original_pos),
        )
    }
}
