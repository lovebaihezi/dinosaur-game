use bevy::math::bounding::Aabb2d;
use bevy::math::bounding::IntersectsVolume;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::components::Dino;
use crate::components::Tree;
use crate::utils::egui_wants_pointer;
use crate::GameScreen;

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
    dino_query: Query<(&Transform, &Sprite), With<Dino>>,
    tree_query: Query<(&Sprite, &Transform), With<Tree>>,
    mut next_screen: ResMut<NextState<GameScreen>>,
) {
    for ((dino_transform, dino_sprite), (tree_sprite, tree_transform)) in
        dino_query.iter().zip(tree_query.iter())
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
) {
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
