use bevy::prelude::*;

pub fn cleanup<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
