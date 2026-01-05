use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::components::{Dino, DINO_DEFAULT_COLOR, DINO_TOUCHED_COLOR};
use crate::utils::egui_wants_pointer;
use crate::{utils::cleanup_component, GameConfig, GameScreen, GameStatus};

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScreen::StartScreen), setup_start_screen_dino)
            .add_systems(
                Update,
                (
                    update_start_dino_position,
                    update_start_dino_sprite_from_config,
                )
                    .run_if(in_state(GameScreen::StartScreen)),
            )
            .add_systems(
                FixedUpdate,
                enter_play_by_space.run_if(in_state(GameScreen::StartScreen)),
            )
            .add_systems(OnExit(GameScreen::StartScreen), cleanup_component::<Dino>);
    }
}

/// Spawn dino on ground at start screen
fn setup_start_screen_dino(mut commands: Commands, config: Res<GameConfig>) {
    info!("Setting up dino on start screen");
    commands.spawn(Dino::new(&config));
}

/// Keep dino positioned correctly on start screen
fn update_start_dino_position(
    mut query: Query<(&mut Transform, &Sprite), With<Dino>>,
    game_status: Res<GameStatus>,
    config: Res<GameConfig>,
) {
    for (mut transform, sprite) in query.iter_mut() {
        let window_width = game_status.window_width;
        let dino_width = sprite.custom_size.map(|s| s.x).unwrap_or(config.dino_width);
        transform.translation.x =
            -window_width / 2.0 + dino_width / 2.0 + config.dino_x_offset * window_width;
    }
}

/// Update dino sprite size based on config changes in real-time for start screen
fn update_start_dino_sprite_from_config(
    mut query: Query<&mut Sprite, With<Dino>>,
    config: Res<GameConfig>,
) {
    for mut sprite in query.iter_mut() {
        let new_size = bevy::math::Vec2::new(config.dino_width, config.dino_height);
        if sprite.custom_size != Some(new_size) {
            sprite.custom_size = Some(new_size);
        }
    }
}

fn enter_play_by_space(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut next_screen: ResMut<NextState<GameScreen>>,
    mut dino_query: Query<(&mut Dino, &mut Sprite)>,
    mut contexts: EguiContexts,
) {
    // Only process mouse/touch if egui doesn't want the input
    let touch_input = if egui_wants_pointer(&mut contexts) {
        false
    } else {
        touches.any_just_pressed()
    };

    let mouse_input = if egui_wants_pointer(&mut contexts) {
        false
    } else {
        mouse.just_pressed(MouseButton::Left)
    };

    // If touch input detected, turn dino red (only one dino expected on start screen)
    if touch_input {
        if let Ok((mut dino, mut sprite)) = dino_query.single_mut() {
            if !dino.is_touched {
                dino.is_touched = true;
                sprite.color = DINO_TOUCHED_COLOR;
                info!("Dino touched - turning red");
            }
        }
    }

    // Space key resets dino to default color (begin state) or starts game
    if keyboard.just_pressed(KeyCode::Space) {
        if let Ok((mut dino, mut sprite)) = dino_query.single_mut() {
            if dino.is_touched {
                // Reset dino to default state
                dino.is_touched = false;
                sprite.color = DINO_DEFAULT_COLOR;
                info!("Dino reset to default state");
            } else {
                // Start playing
                info!("Start Playing");
                next_screen.set(GameScreen::PlayScreen);
            }
        } else {
            // No dino found, just start playing
            info!("Start Playing");
            next_screen.set(GameScreen::PlayScreen);
        }
    }

    // Mouse click starts the game directly (not touch, which turns dino red first)
    if mouse_input {
        info!("Start Playing via mouse click");
        next_screen.set(GameScreen::PlayScreen);
    }
}
