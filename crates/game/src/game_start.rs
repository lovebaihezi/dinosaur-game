use bevy::prelude::*;

use crate::{utils::cleanup_component, GameScreen};

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScreen::StartScreen), show_start_message)
            .add_systems(
                FixedUpdate,
                enter_play_by_space.run_if(in_state(GameScreen::StartScreen)),
            )
            .add_systems(
                OnExit(GameScreen::StartScreen),
                cleanup_component::<WelcomeTextUI>,
            );
    }
}

#[derive(Component)]
struct WelcomeTextUI;

fn show_start_message(mut commands: Commands) {
    info!("Showing start message");
    commands
        .spawn((
            WelcomeTextUI,
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
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Press Space/Touch/Click to Start!"),
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

fn enter_play_by_space(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut next_screen: ResMut<NextState<GameScreen>>,
) {
    if keyboard.just_pressed(KeyCode::Space)
        || touches.any_just_pressed()
        || mouse.just_pressed(MouseButton::Left)
    {
        info!("Start Playing");
        next_screen.set(GameScreen::PlayScreen);
    }
}
