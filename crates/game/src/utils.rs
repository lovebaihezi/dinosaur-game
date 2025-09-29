use bevy::prelude::*;

pub fn cleanup_component<C: Component>(querys: Query<Entity, With<C>>, mut commands: Commands) {
    for entity in querys {
        commands.entity(entity).remove::<C>();
    }
}
