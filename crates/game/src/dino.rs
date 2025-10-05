use bevy::{
    app::{Plugin, Update},
    asset::AssetServer,
    audio::{AudioPlayer, AudioSink, AudioSinkPlayback, PlaybackSettings},
    ecs::schedule::IntoScheduleConfigs,
    input::ButtonInput,
    log::info,
    prelude::{Commands, KeyCode, MouseButton, Query, Res, Touches, Transform, With},
    sprite::Sprite,
    state::{condition::in_state, state::OnEnter},
    time::{Time, Virtual},
};

use crate::{components::Dino, utils::cleanup_component, DinoJumpMusic, GameScreen, GameStatus};

pub struct DinoPlugin;

impl Plugin for DinoPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            (dino_pos_fix_system, dino_jump_system, dino_jump_animation)
                .run_if(in_state(GameScreen::PlayScreen)),
        )
        .add_systems(OnEnter(GameScreen::PlayScreen), setup_dino)
        .add_systems(
            OnEnter(GameScreen::GameOverScreen),
            (cleanup_component::<Dino>, clean_dino_jump_music),
        );
    }
}

fn setup_dino(mut commands: Commands, assert_server: Res<AssetServer>) {
    let sound = assert_server.load("Jump.ogg");
    commands.insert_resource(DinoJumpMusic(sound));
    commands.spawn(Dino::new());
}

fn clean_dino_jump_music(mut commands: Commands) {
    commands.remove_resource::<DinoJumpMusic>();
}

fn dino_pos_fix_system(
    mut query: Query<(&mut Transform, &Sprite), With<Dino>>,
    game_status: Res<GameStatus>,
) {
    for (mut transform, _sprite) in query.iter_mut() {
        let window_width = game_status.window_width;
        transform.translation.x = -window_width / 2.0 + Dino::WIDTH / 2.0 + 0.2 * window_width;
    }
}

/// Dino will jump when user press space, w, Up, k, or left mouse button
fn dino_jump_system(
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

fn dino_jump_animation(
    time: Res<Time<Virtual>>,
    mut query: Query<(&mut Transform, &mut Dino)>,
    sink: Query<&AudioSink, With<Dino>>,
) {
    if time.is_paused() {
        return;
    }
    for (mut transform, mut dino) in query.iter_mut() {
        if let Some(start_time) = dino.in_air_start_time {
            let elapsed = time.elapsed() - start_time.elapsed();
            // Over
            let y = if elapsed.as_millis() > 500 {
                if let Ok(sink) = sink.single() {
                    info!("Pause Sink");
                    sink.pause();
                }
                dino.in_air_start_time = None;
                Dino::WIDTH / 2.0 / 0.618
            } else {
                let x = elapsed.as_millis() as f64 / 500.0 * std::f64::consts::PI;
                let x = x as f32;
                x.sin() * Dino::JUMP_HIGH + Dino::WIDTH / 2.0 / 0.618
            };
            transform.translation.y = y;
        }
    }
}
