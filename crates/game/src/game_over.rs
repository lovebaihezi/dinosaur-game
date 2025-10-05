use bevy::prelude::*;

use crate::{utils::cleanup_component, GameScreen};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameScreen::GameOverScreen), show_game_over_info)
            .add_systems(
                FixedUpdate,
                restart_game_by_space.run_if(in_state(GameScreen::GameOverScreen)),
            )
            .add_systems(
                OnExit(GameScreen::GameOverScreen),
                cleanup_component::<GameOverTextUI>,
            );
    }
}

#[derive(Component)]
pub struct GameOverTextUI;

fn show_game_over_info(mut commands: Commands) {
    info!("Showing Game Over");
    commands
        .spawn((
            GameOverTextUI,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                column_gap: Val::Px(16.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            BackgroundColor(Color::linear_rgba(1.0, 1.0, 1.0, 0.8)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Press Space/Touch/Click to Restart!"),
                TextFont {
                    font_size: 48.0,
                    ..Default::default()
                },
                TextLayout {
                    justify: JustifyText::Center,
                    ..Default::default()
                },
                TextColor(Color::BLACK),
            ));
        });
}

fn restart_game_by_space(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut next_state: ResMut<NextState<GameScreen>>,
) {
    info!("KEY");
    if keyboard.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || touches.any_just_pressed()
    {
        info!("Restart Game");
        next_state.set(GameScreen::StartScreen);
    }
}
