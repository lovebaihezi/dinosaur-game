use bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct GameStatus {
    pub score: u64,
    pub speed: u64,
    pub window_width: f32,
    pub window_height: f32,
}

#[derive(Debug, Default, Resource)]
pub struct SpeedControlInfo {
    pub speed_increment: u64,
    pub max_game_speed: u64,
}

#[derive(Resource, Deref)]
pub struct DinoJumpMusic(pub Handle<bevy_kira_audio::AudioSource>);
