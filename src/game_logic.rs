use bevy::math::bounding::Aabb2d;
use bevy::math::bounding::IntersectsVolume;
use bevy::prelude::*;

use crate::components::Dino;
use crate::components::Tree;
use crate::components::DINO_HEIGHT;
use crate::components::TREE_WIDTH;

pub fn dino_touched_tree(
    mut dino_query: Query<(&Sprite, &mut Dino, &Transform)>,
    tree_query: Query<(&Sprite, &Transform), With<Tree>>,
    mut time: ResMut<Time<Virtual>>,
) {
    if time.is_paused() {
        return;
    }
    for ((dino_sprite, mut entity, dino_transform), (tree_sprite, tree_transform)) in
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
            time.pause();
            entity.over();
        }
    }
}

pub fn reset_game(
    mut dino_query: Query<(&mut Dino, &mut Transform), Without<Tree>>,
    mut tree_query: Query<(&mut Tree, &mut Transform), Without<Dino>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touch: Res<Touches>,
    window: Query<&Window>,
) {
    let press_reset = keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::KeyW)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::KeyK)
        || mouse.just_pressed(MouseButton::Left)
        || touch.any_just_pressed();
    for ((mut dino, mut dino_transform), (mut tree, mut tree_transform)) in
        dino_query.iter_mut().zip(tree_query.iter_mut())
    {
        if dino.is_over() && press_reset {
            dino.ready();
            tree.ready();
            let window = window.single();
            let window_width = window.width();
            tree_transform.translation.x = window_width - TREE_WIDTH;
            dino_transform.translation.y = DINO_HEIGHT / 2.0;
        }
    }
}

#[cfg(test)]
mod game_logic_test {
    use bevy::prelude::*;

    use crate::{
        components::Dino,
        dino_jump_animation, dino_jump_system, dino_pos_fix_system, game_info,
        game_logic::{dino_touched_tree, reset_game},
        setup_camera, setup_dino, setup_game_control, setup_ground, setup_tree,
        tree_move_animation, update_ground, user_control, GameStatus,
    };
    #[test]
    fn time_paused_on() {
        let mut app = App::new();
        app.insert_resource(GameStatus { speed: 5, score: 0 });
        app.insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)));
        app.add_systems(
            Startup,
            (
                setup_ground,
                setup_dino,
                setup_camera,
                setup_tree,
                setup_game_control,
            ),
        );
        app.add_systems(
            Update,
            (
                update_ground,
                dino_jump_system,
                (user_control, game_info).chain(),
                (dino_pos_fix_system, dino_jump_animation).chain(),
                tree_move_animation,
                (dino_touched_tree, reset_game).chain(),
            ),
        );

        // Setup test entities
        let enemy_id = app.world_mut().spawn(Dino::default()).id();

        // Run systems
        app.update();

        // Check resulting changes
        assert!(app.world().get::<Dino>(enemy_id).is_some());
        assert!(app
            .world()
            .get::<Dino>(enemy_id)
            .unwrap()
            .in_air_start_time
            .is_none());
    }
}
