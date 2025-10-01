use bevy::prelude::*;

pub fn cleanup_component<C: Component>(queries: Query<Entity, With<C>>, mut commands: Commands) {
    for entity in queries {
        commands.entity(entity).remove::<C>();
    }
}
