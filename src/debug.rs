use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ray_velocity);
    }
}

fn ray_velocity(query: Query<&Velocity, With<RigidBody>>) {
    for velocity in query.iter() {
        println!("Ball Velocity:{:?}", velocity)
    }
}
