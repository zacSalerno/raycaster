use bevy::prelude::*;

use crate::{
    constants::HEIGHT,
    constants::WIDTH,
    ray::RADIUS,
    ray::{Acceleration, Velocity},
};

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, border_collide);
    }
}

fn border_collide(mut query: Query<(&mut Velocity, &mut Acceleration, &mut Transform)>) {
    for (mut velocity, mut acceleration, transform) in query.iter_mut() {
        if transform.translation.x <= -WIDTH / 2.0 + RADIUS {
            velocity.value.x = -velocity.value.x;
            acceleration.value.x = -acceleration.value.x;
        }
        if transform.translation.x >= WIDTH / 2.0 - RADIUS {
            velocity.value.x = -velocity.value.x;
            acceleration.value.x = -acceleration.value.x;
        }
        if transform.translation.y <= -HEIGHT / 2.0 + RADIUS {
            velocity.value.y = -velocity.value.y;
            acceleration.value.y = -acceleration.value.y;
        }
        if transform.translation.y >= HEIGHT / 2.0 - RADIUS {
            velocity.value.y = -velocity.value.y;
            acceleration.value.y = -acceleration.value.y;
        }
    }
}
