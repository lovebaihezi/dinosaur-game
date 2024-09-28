use bevy::{
    prelude::Component,
    time::{Time, Virtual},
};

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
    FPS,
    Score,
    Tip,
}

#[derive(Component)]
pub struct Ground;

#[derive(Component, Default)]
pub struct Tree {
    succeeded: usize,
}

impl Tree {
    pub fn dino_passed(&mut self) {
        self.succeeded += 1;
    }
    pub fn score(&self) -> usize {
        self.succeeded
    }
    pub fn speed(&mut self) -> f64 {
        (self.succeeded as f64 + 2.2).ln()
    }
}
