use bevy::{
    math::Vec2,
    prelude::Component,
    time::{Time, Virtual},
};

pub const TREE_WIDTH: f32 = 30.0;
pub const DINO_WIDTH: f32 = 50.0;
pub const DINO_HEIGHT: f32 = DINO_WIDTH / 0.618;
pub const DINO_SIZE: Vec2 = Vec2::new(DINO_WIDTH, DINO_WIDTH / 0.618);
pub const JUMP_HIGH: f32 = DINO_WIDTH / 0.618 * 2.4;

#[derive(Default)]
pub enum GameState {
    #[default]
    Ready,
    Playing,
    Over,
}

#[derive(Component, Default)]
pub struct Dino {
    pub in_air_start_time: Option<Time<Virtual>>,
    state: GameState,
}

impl Dino {
    pub fn ready(&mut self) {
        self.in_air_start_time = None;
        self.state = GameState::Ready;
    }
    pub fn start(&mut self) {
        self.state = GameState::Playing;
    }
    pub fn over(&mut self) {
        self.state = GameState::Over;
    }
    pub fn is_ready(&self) -> bool {
        matches!(self.state, GameState::Ready)
    }
    pub fn is_playing(&self) -> bool {
        matches!(self.state, GameState::Playing)
    }
    pub fn is_over(&self) -> bool {
        matches!(self.state, GameState::Over)
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
    pub fn ready(&mut self) {}
    pub fn start(&mut self) {}
}
