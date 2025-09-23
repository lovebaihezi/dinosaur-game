use bevy::state::state::States;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, States)]
pub enum GameScreen {
    #[default]
    StartScreen,
    // Click play to
    PlayScreen,
    // Unfocus
    UnfocusedPauseScreen,
    // Esc
    ManuallyPauseScreen,
    // Dino touchs tree
    GameOverScreen,
    // End State
    ExitScreen,
}
