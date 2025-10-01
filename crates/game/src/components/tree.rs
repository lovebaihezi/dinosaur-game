use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    prelude::Component,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

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
