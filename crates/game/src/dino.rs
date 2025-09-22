use bevy::{
    app::{Plugin, Update},
    asset::AssetServer,
    audio::{AudioPlayer, AudioSink, AudioSinkPlayback, PlaybackSettings},
    ecs::{entity::Entity, schedule::IntoScheduleConfigs},
    input::ButtonInput,
    log::info,
    prelude::{Commands, KeyCode, MouseButton, Query, Res, Touches, Transform, With},
    sprite::Sprite,
    state::{
        condition::in_state,
        state::{OnEnter, OnExit, OnTransition},
    },
    time::{Time, Virtual},
};

use crate::{components::Dino, DinoJumpMusic, GameScreen, GameStatus};

pub struct DinoPlugin;

impl Plugin for DinoPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            (dino_pos_fix_system, dino_jump_system, dino_jump_animation)
                .run_if(in_state(GameScreen::PlayScreen)),
        )
        .add_systems(OnEnter(GameScreen::PlayScreen), setup_dino)
        // TODO: the state should optimzed to DFA
        .add_systems(
            OnTransition {
                exited: GameScreen::PlayScreen,
                entered: GameScreen::GameOverScreen,
            },
            cleanup_dino,
        )
        .add_systems(
            OnTransition {
                exited: GameScreen::ManuallyPauseScreen,
                entered: GameScreen::GameOverScreen,
            },
            cleanup_dino,
        )
        .add_systems(
            OnTransition {
                exited: GameScreen::UnfocusedPauseScreen,
                entered: GameScreen::GameOverScreen,
            },
            cleanup_dino,
        );
    }
}

pub fn setup_dino(mut commands: Commands, assert_server: Res<AssetServer>) {
    let sound = assert_server.load("Jump.ogg");
    commands.insert_resource(DinoJumpMusic(sound));
    commands.spawn(Dino::new());
}

pub fn cleanup_dino(mut commands: Commands, dinos: Query<Entity, With<Dino>>) {
    commands.remove_resource::<DinoJumpMusic>();
    for entity in dinos {
        commands.entity(entity).remove::<Dino>();
    }
}

pub fn dino_pos_fix_system(
    mut query: Query<(&mut Transform, &Sprite), With<Dino>>,
    game_status: Res<GameStatus>,
) {
    for (mut transform, _sprite) in query.iter_mut() {
        let window_width = game_status.window_width;
        transform.translation.x = -window_width / 2.0 + Dino::DINO_WIDTH / 2.0 + 0.2 * window_width;
    }
}

/// Dino will jump when user press space, w, Up, k, or left mouse button
pub fn dino_jump_system(
    mut dino_query: Query<&mut Dino>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touch: Res<Touches>,
    time: Res<Time<Virtual>>,
    mut commands: Commands,
    sound: Res<DinoJumpMusic>,
) {
    if time.is_paused() {
        return;
    }
    if keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::KeyW)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::KeyK)
        || mouse.just_pressed(MouseButton::Left)
        || touch.any_just_pressed()
    {
        for mut dino in dino_query.iter_mut() {
            if dino.in_air_start_time.is_some() {
                continue;
            } else {
                commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
                dino.in_air_start_time = Some(*time);
            }
        }
    }
}

pub fn dino_jump_animation(
    time: Res<Time<Virtual>>,
    mut query: Query<&mut Dino>,
    sink: Query<&AudioSink, With<Dino>>,
) {
    if time.is_paused() {
        return;
    }
    for mut dino in query.iter_mut() {
        if let Some(start_time) = dino.in_air_start_time {
            let elapsed = time.elapsed() - start_time.elapsed();
            // Over
            let y = if elapsed.as_millis() > 500 {
                if let Ok(sink) = sink.single() {
                    info!("Pause Sink");
                    sink.pause();
                }
                dino.in_air_start_time = None;
                Dino::DINO_WIDTH / 2.0 / 0.618
            } else {
                let x = elapsed.as_millis() as f64 / 500.0 * std::f64::consts::PI;
                let x = x as f32;
                x.sin() * Dino::JUMP_HIGH + Dino::DINO_WIDTH / 2.0 / 0.618
            };
            dino.transform.translation.y = y;
        }
    }
}
