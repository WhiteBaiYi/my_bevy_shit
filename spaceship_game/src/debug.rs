use bevy::prelude::*;

pub fn print_position (
    query: Query<(Entity, &Transform)>
) {
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} is at position {:?},",
            entity, transform.translation
        );
    }
}