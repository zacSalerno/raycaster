use bevy::prelude::*;

use crate::ray::Velocity;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

fn print_position(query: Query<(Entity, &Transform, &Velocity)>) {
    for (entity, transform, velocity) in query.iter() {
        info!(
            "Entity {:?} is at position {:?} with this velocity {:?}",
            entity, transform.translation, velocity
        );
    }
}
