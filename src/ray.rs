use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

const STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(-100.0, -100.0, 0.0);
const STARTING_ACCELERATION: Vec3 = Vec3::new(-100.0, -100.0, 0.0);
pub const RADIUS: f32 = 20.0;

pub struct RayPlugin;
impl Plugin for RayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ray);
    }
}

#[derive(Bundle)]
pub struct RayBundle<M: bevy::sprite::Material2d> {
    velocity: Velocity,
    acceleration: Acceleration,
    model: MaterialMesh2dBundle<M>,
}

fn spawn_ray(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(RayBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        acceleration: Acceleration {
            value: STARTING_ACCELERATION,
        },
        model: MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(RADIUS)).into(),
            transform: Transform {
                translation: STARTING_POSITION,
                ..Default::default()
            },
            material: materials.add(Color::hex("#e1c19b").unwrap()),
            ..default()
        },
    });
}
