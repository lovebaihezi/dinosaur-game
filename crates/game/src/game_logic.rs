use bevy::math::bounding::Aabb2d;
use bevy::math::bounding::IntersectsVolume;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::components::Tree;
use crate::components::{Dino, DINO_TOUCHED_COLOR};
use crate::utils::egui_wants_pointer;
use crate::{BlurAnimationState, GameScreen};

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            FixedUpdate,
            (
                dino_touched_tree.run_if(in_state(GameScreen::PlayScreen)),
                back_to_play_while_game_over.run_if(in_state(GameScreen::GameOverScreen)),
            ),
        );
    }
}

fn dino_touched_tree(
    mut dino_query: Query<(&Transform, &mut Sprite, &mut Dino)>,
    tree_query: Query<(&Sprite, &Transform), (With<Tree>, Without<Dino>)>,
    mut next_screen: ResMut<NextState<GameScreen>>,
) {
    for ((dino_transform, mut dino_sprite, mut dino), (tree_sprite, tree_transform)) in
        dino_query.iter_mut().zip(tree_query.iter())
    {
        let aabb_dino = Aabb2d::new(
            dino_transform.translation.xy(),
            dino_sprite.custom_size.unwrap() / 2.0 / dino_transform.scale.xy(),
        );

        let aabb_tree = Aabb2d::new(
            tree_transform.translation.xy(),
            tree_sprite.custom_size.unwrap() / 2.0 / tree_transform.scale.xy(),
        );

        if aabb_tree.intersects(&aabb_dino) {
            // Turn dino red when touched
            dino.is_touched = true;
            dino_sprite.color = DINO_TOUCHED_COLOR;
            next_screen.set(GameScreen::GameOverScreen);
        }
    }
}

fn back_to_play_while_game_over(
    cur_screen: Res<State<GameScreen>>,
    mut next_screen: ResMut<NextState<GameScreen>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touch: Res<Touches>,
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
        touch.any_just_pressed() || mouse.just_pressed(MouseButton::Left)
    };

    if *cur_screen == GameScreen::GameOverScreen
        && (keyboard.just_pressed(KeyCode::Space) || pointer_input)
    {
        next_screen.set(GameScreen::PlayScreen);
    }
}
