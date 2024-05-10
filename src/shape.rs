use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{HEIGHT, WIDTH};

pub struct ShapePlugin;
impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shape);
    }
}

fn spawn_shape(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let vertices: Vec<Vect> = vec![
        (150.0, 150.0).into(),
        (150.0, -150.0).into(),
        (-150.0, -150.0).into(),
        (-150.0, 150.0).into(),
        (150.0, 150.0).into(),
    ];

    commands
        .spawn(Collider::polyline(vertices, None))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(1.0))
        .insert(ColliderMassProperties::Mass(1000000.0));
}
