use bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct GameStatus {
    pub score: u64,
    pub speed: u64,
}
