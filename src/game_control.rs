use bevy::{
    color::Color,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::ButtonInput,
    math::Vec3,
    prelude::{
        BuildChildren, Commands, KeyCode, Local, MouseButton, NodeBundle, Query, Res, ResMut,
        TextBundle, Touches, Transform,
    },
    text::{Text, TextStyle},
    time::{Time, Virtual},
    ui::Style,
    utils::default,
    window::Window,
};

use crate::components::{Dino, GameControl};

pub fn setup_game_control(
    mut commands: Commands,
    mut time: ResMut<Time<Virtual>>,
    window: Query<&Window>,
) {
    time.pause();

    let window = window.single();
    let window_width = window.width();
    let window_height = window.height();

    const GAME_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-", env!("GIT_HASH"));

    commands
        .spawn(NodeBundle {
            style: Style {
                width: bevy::ui::Val::Px(100.0),
                height: bevy::ui::Val::Px(100.0),
                align_items: bevy::ui::AlignItems::Center,
                justify_content: bevy::ui::JustifyContent::Center,
                display: bevy::ui::Display::Flex,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                -window_width / 2.0,
                window_height / 2.0,
                0.0,
            )),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    align_self: bevy::ui::AlignSelf::Center,
                    ..default()
                },
                text: Text::from_section(
                    GAME_VERSION,
                    TextStyle {
                        color: Color::srgba(0.0, 1.0, 1.0, 0.8),
                        ..default()
                    },
                ),
                ..default()
            });
        });

    commands
        .spawn((NodeBundle {
            style: Style {
                width: bevy::ui::Val::Vw(100.0),
                height: bevy::ui::Val::Auto,
                align_items: bevy::ui::AlignItems::Center,
                justify_content: bevy::ui::JustifyContent::SpaceAround,
                display: bevy::ui::Display::Flex,
                ..default()
            },
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
                    text: Text::from_section(
                        "ERROR",
                        TextStyle {
                            color: Color::srgba(0.0, 0.0, 0.0, 0.96),
                            ..default()
                        },
                    ),
                    ..default()
                },
                GameControl::FPS,
            ));
            parent.spawn((
                TextBundle {
                    style: Style {
                        align_self: bevy::ui::AlignSelf::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        "ERROR",
                        TextStyle {
                            color: Color::srgba(0.0, 0.0, 0.0, 0.96),
                            ..default()
                        },
                    ),
                    ..default()
                },
                GameControl::Score,
            ));
            // parent.spawn((
            //     TextBundle {
            //         style: Style {
            //             align_self: bevy::ui::AlignSelf::Center,
            //             ..default()
            //         },
            //         text: Text::from_section(
            //             "ERROR",
            //             TextStyle {
            //                 color: Color::srgba(0.0, 0.0, 0.0, 0.96),
            //                 ..default()
            //             },
            //         ),
            //         ..default()
            //     },
            //     GameControl::Tip,
            // ));
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

pub fn game_info(
    mut text_query: Query<(&mut Text, &GameControl)>,
    dino_query: Query<&Dino>,
    diagnostics: Res<DiagnosticsStore>,
    mut score: Local<u64>,
    time: Res<Time<Virtual>>,
) {
    if !time.is_paused() {
        *score += 1;
    }
    for dino in dino_query.iter() {
        if dino.is_over() {
            *score = 0;
        }
        for (mut text, game_control) in text_query.iter_mut() {
            match game_control {
                GameControl::Score => {
                    let value: u64 = *score >> 3;
                    text.sections[0].value = format!("{value:012}");
                }
                GameControl::FPS => {
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
        }
    }
}
