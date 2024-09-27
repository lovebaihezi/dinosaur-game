use bevy::{
    color::Color,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::ButtonInput,
    log::info,
    math::Vec3,
    prelude::{
        BuildChildren, Commands, KeyCode, MouseButton, NodeBundle, Query, Res, ResMut, TextBundle,
        Touches, Transform, With,
    },
    text::{Text, TextSection, TextStyle},
    time::{Time, Virtual},
    ui::{GridPlacement, Style},
    utils::default,
    window::Window,
};

use crate::components::{Dino, GameControl, Tree};

pub fn setup_game_control(
    mut commands: Commands,
    mut time: ResMut<Time<Virtual>>,
    window: Query<&Window>,
) {
    time.pause();

    let window = window.single();
    let window_width = window.width();
    let window_height = window.height();

    commands
        .spawn((NodeBundle {
            style: Style { ..default() },
            transform: Transform::from_translation(Vec3::new(
                -window_width / 2.0,
                -window_height / 2.0,
                0.0,
            )),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style {
                        align_self: bevy::ui::AlignSelf::Center,
                        ..default()
                    },
                    text: Text::from_sections([
                        TextSection::new(
                            "FPS",
                            TextStyle {
                                color: Color::srgba(0.0, 0.0, 0.0, 0.96),
                                font_size: 12.0,
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "GAME INFO",
                            TextStyle {
                                color: Color::srgba(0.0, 0.0, 0.0, 0.96),
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "TIP",
                            TextStyle {
                                color: Color::srgba(0.0, 1.0, 0.0, 0.95),
                                font_size: 12.0,
                                ..default()
                            },
                        ),
                    ]),
                    ..default()
                },
                GameControl {},
            ));
        });
}

pub fn user_control(
    mut time: ResMut<Time<Virtual>>,
    mut dino_query: Query<&mut Dino>,
    window: Query<&Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
) {
    let window = window.single();
    for mut dino in dino_query.iter_mut() {
        if window.focused
            && time.is_paused()
            && (keyboard.just_pressed(KeyCode::Space)
                || touches.any_just_pressed()
                || mouse.just_pressed(MouseButton::Left))
        {
            dino.start();
            time.unpause();
        } else if !window.focused && !time.is_paused() {
            time.pause();
        };
    }
}

/// Update the ground width, position on window resize, fps and control info
pub fn fps_info(
    mut text_query: Query<&mut Text, With<GameControl>>,
    dino_query: Query<&Dino>,
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time<Virtual>>,
) {
    for (mut text, dino) in text_query.iter_mut().zip(dino_query.iter()) {
        let (fps, avg, smoothed) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .map(|x| {
                (
                    x.value().unwrap_or_default(),
                    x.average().unwrap_or_default(),
                    x.smoothed().unwrap_or_default(),
                )
            })
            .unwrap_or_default();
        let fps_info = format!("{fps:.0}|{avg:.0}|{smoothed:.0}\n");
        text.sections[0].value = fps_info;

        let game_info = format!(
            "Score: {score:020}, State: {state}\n",
            score = time.elapsed().as_millis() / 50,
            state = if time.is_paused() {
                "Paused"
            } else {
                "Running"
            }
        );
        text.sections[1].value = game_info;

        let tip = if dino.is_over() {
            "Game Over! Press Space to restart"
        } else if dino.is_ready() {
            "Press Space to start"
        } else {
            "Jump!"
        };
        text.sections[2].value = tip.to_owned();
    }
}
