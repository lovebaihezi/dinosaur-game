use bevy::{
    color::Color,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::ButtonInput,
    math::Vec3,
    prelude::{
        BuildChildren, Commands, KeyCode, MouseButton, NodeBundle, Query, Res, ResMut, TextBundle,
        Touches, Transform, With,
    },
    text::{Text, TextSection, TextStyle},
    time::{Time, Virtual},
    ui::Style,
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
            let fps_info = TextSection::new(
                "FPS",
                TextStyle {
                    color: Color::srgba(0.0, 0.5, 0.9, 0.96),
                    font_size: 14.0,
                    ..default()
                },
            );
            let game_info = TextSection::new(
                "GAME INFO",
                TextStyle {
                    color: Color::srgba(0.0, 0.0, 0.0, 0.96),
                    font_size: 16.0,
                    ..default()
                },
            );
            let tip = TextSection::new(
                "TIP",
                TextStyle {
                    color: Color::srgba(0.0, 0.0, 0.0, 0.96),
                    font_size: 20.0,
                    ..default()
                },
            );
            parent.spawn((
                TextBundle {
                    style: Style {
                        align_self: bevy::ui::AlignSelf::Center,
                        ..default()
                    },
                    text: Text::from_sections([fps_info, game_info, tip]),
                    ..default()
                },
                GameControl {},
            ));
        });
}

pub fn user_control(
    mut time: ResMut<Time<Virtual>>,
    window: Query<&Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
) {
    let window = window.single();
    if window.focused
        && time.is_paused()
        && (keyboard.just_pressed(KeyCode::Space)
            || touches.any_just_pressed()
            || mouse.just_pressed(MouseButton::Left))
    {
        time.unpause();
    } else if !window.focused && !time.is_paused() {
        time.pause();
    };
}

/// Update the ground width, position on window resize, fps and control info
pub fn fps_info(
    mut query: Query<&mut Text, With<GameControl>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut text in query.iter_mut() {
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
        let fps_info = format!("{fps:.0}|{avg:.0}|{smoothed:.0}");
        text.sections[0].value = fps_info;
    }
}

pub fn game_info(
    mut query: Query<(&mut Text, &Tree), With<GameControl>>,
    time: Res<Time<Virtual>>,
) {
    for (mut text, tree) in query.iter_mut() {
        let game_info = format!(
            "Score: {score}, State: {state}",
            score = tree.score(),
            state = if time.is_paused() {
                "Paused"
            } else {
                "Running"
            }
        );
        text.sections[1].value = game_info;
    }
}

pub fn tip(mut query: Query<(&mut Text, &Dino), With<GameControl>>) {
    for (mut text, dino) in query.iter_mut() {
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
