use bevy::prelude::*;

use crate::{utils::cleanup_component, GameScreen};

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScreen::StartScreen), show_start_message)
            .add_systems(
                FixedUpdate,
                (
                    enter_play_by_space.run_if(in_state(GameScreen::StartScreen)),
                    change_text_color_by_time.run_if(in_state(GameScreen::StartScreen)),
                ),
            )
            .add_systems(
                OnExit(GameScreen::StartScreen),
                cleanup_component::<WelcomeTextUI>,
            );
    }
}

#[derive(Component)]
struct WelcomeTextUI {
    pub node: Node,
    pub text: Text,
    pub text_font: TextFont,
    pub text_layout: TextLayout,
    pub text_color: TextColor,
}

impl WelcomeTextUI {
    fn new() -> Self {
        Self {
            node: Node {
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
            text: Text::new("Press Space/Touch/Click to Start!"),
            text_font: TextFont {
                font_size: 48.0,
                ..Default::default()
            },
            text_layout: TextLayout {
                justify: JustifyText::Center,
                ..Default::default()
            },
            text_color: TextColor(Color::BLACK),
        }
    }
}

fn show_start_message(mut commands: Commands) {
    commands.spawn(WelcomeTextUI::new());
}

fn change_text_color_by_time(time: Res<Time<Virtual>>, mut query: Query<&mut WelcomeTextUI>) {
    for mut welcome_ui in query.iter_mut() {
        let seconds = time.elapsed_secs().sin() * 0.5 + 0.5;
        welcome_ui.text_color.0 = Color::linear_rgb(seconds, seconds, seconds);
    }
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
        next_screen.set(GameScreen::PlayScreen);
    }
}
