use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::utils::egui_wants_pointer;
use crate::{utils::cleanup_component, BlurAnimationState, GameConfig, GameScreen};

/// Retry icon unicode character (clockwise arrow)
const RETRY_ICON: &str = "⟳";

/// Exponent for ease-out animation curve (higher = faster start, slower end)
const EASE_OUT_POWER: i32 = 2;

/// Maximum blur overlay size in percent (200% covers full screen when centered at 50%)
const MAX_BLUR_SIZE_PERCENT: f32 = 200.0;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameScreen::GameOverScreen), setup_game_over)
            .add_systems(
                Update,
                (update_blur_animation, update_game_over_ui)
                    .run_if(in_state(GameScreen::GameOverScreen)),
            )
            .add_systems(FixedUpdate, restart_game_by_space)
            .add_systems(
                OnExit(GameScreen::GameOverScreen),
                (
                    cleanup_component::<GameOverTextUI>,
                    cleanup_component::<BlurOverlay>,
                    cleanup_blur_animation_state,
                ),
            );
    }
}

#[derive(Component)]
pub struct GameOverTextUI;

/// Component to mark the blur overlay
#[derive(Component)]
pub struct BlurOverlay;

fn cleanup_blur_animation_state(mut commands: Commands) {
    commands.remove_resource::<BlurAnimationState>();
}

fn setup_game_over(mut commands: Commands) {
    info!("Showing Game Over");

    // Initialize blur animation state
    commands.insert_resource(BlurAnimationState::default());

    // Create the blur overlay that expands from center
    commands.spawn((
        BlurOverlay,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Percent(0.0),
            height: Val::Percent(0.0),
            ..Default::default()
        },
        BackgroundColor(Color::linear_rgba(1.0, 1.0, 1.0, 0.9)),
        BorderRadius::all(Val::Percent(50.0)),
        ZIndex(10),
    ));

    // Create the game over UI container with retry icon
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
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            BackgroundColor(Color::NONE),
            ZIndex(20),
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            // Retry icon with border container
            parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(120.0),
                        height: Val::Px(120.0),
                        border: UiRect::all(Val::Px(4.0)),
                        ..Default::default()
                    },
                    BorderColor::all(Color::BLACK),
                    BorderRadius::all(Val::Px(16.0)),
                    BackgroundColor(Color::linear_rgba(1.0, 1.0, 1.0, 0.95)),
                ))
                .with_children(|icon_parent| {
                    icon_parent.spawn((
                        Text::new(RETRY_ICON),
                        TextFont {
                            font_size: 72.0,
                            ..Default::default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            ..Default::default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });

            // Instruction text below the retry icon
            parent.spawn((
                Text::new("Press Space/Touch/Click to Restart"),
                TextFont {
                    font_size: 32.0,
                    ..Default::default()
                },
                TextLayout {
                    justify: Justify::Center,
                    ..Default::default()
                },
                TextColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ));
        });
}

fn update_blur_animation(
    time: Res<Time>,
    config: Res<GameConfig>,
    mut blur_state: ResMut<BlurAnimationState>,
    mut blur_query: Query<&mut Node, With<BlurOverlay>>,
) {
    if blur_state.completed {
        return;
    }

    // Update animation progress
    let duration = config.blur_animation_duration.max(0.1);
    blur_state.progress = (blur_state.progress + time.delta_secs() / duration).min(1.0);

    // Update blur overlay size - expand from center
    for mut node in blur_query.iter_mut() {
        // Use easing function for smooth animation (ease-out)
        let eased_progress = 1.0 - (1.0 - blur_state.progress).powi(EASE_OUT_POWER);

        // Calculate size: starts at 0%, ends at enough to cover the full screen from center
        let size = eased_progress * MAX_BLUR_SIZE_PERCENT;

        // Center the expanding circle by offsetting left and top
        let offset = 50.0 - size / 2.0;

        node.width = Val::Percent(size);
        node.height = Val::Percent(size);
        node.left = Val::Percent(offset);
        node.top = Val::Percent(offset);
    }

    if blur_state.progress >= 1.0 {
        blur_state.completed = true;
    }
}

fn update_game_over_ui(
    blur_state: Res<BlurAnimationState>,
    mut ui_query: Query<&mut Visibility, With<GameOverTextUI>>,
) {
    // Show the game over UI only after blur animation completes
    if blur_state.completed {
        for mut visibility in ui_query.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn restart_game_by_space(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    cur_screen: Res<State<GameScreen>>,
    mut next_state: ResMut<NextState<GameScreen>>,
    mut contexts: EguiContexts,
    blur_state: Option<Res<BlurAnimationState>>,
) {
    // Only allow restart after blur animation completes
    if let Some(state) = blur_state {
        if !state.completed {
            return;
        }
    }

    // Only process mouse/touch if egui doesn't want the input
    let pointer_input = if egui_wants_pointer(&mut contexts) {
        false
    } else {
        mouse.just_pressed(MouseButton::Left) || touches.any_just_pressed()
    };

    if *cur_screen == GameScreen::GameOverScreen
        && (keyboard.just_pressed(KeyCode::Space) || pointer_input)
    {
        info!("Restart Game");
        next_state.set(GameScreen::StartScreen);
    }
}
