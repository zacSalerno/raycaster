use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

const STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const STARTING_VELOCITY: Vec2 = Vec2::new(50.0, 100.0);
const STARTING_ACCELERATION: Vec3 = Vec3::new(-100.0, -100.0, 0.0);
pub const RADIUS: f32 = 2.0;

pub struct RayPlugin;
impl Plugin for RayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ray);
    }
}

fn spawn_ray(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(RADIUS))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Mass(5000.0))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(1.0))
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        })
        .insert(GravityScale(0.0))
        .insert(Ccd::enabled())
        .insert(Velocity {
            linvel: STARTING_VELOCITY,
            angvel: 0.0,
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(RADIUS)).into(),
            transform: Transform {
                translation: STARTING_POSITION,
                ..Default::default()
            },
            material: materials.add(Color::hex("#e1c19b").unwrap()),
            ..default()
        });
}
