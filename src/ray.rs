use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

const STARTING_VELOCITY: Vec3 = Vec3::new(1.0, 1.0, 0.0);
const STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

pub struct RayPlugin;
impl Plugin for RayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ray);
    }
}

#[derive(Bundle)]
pub struct RayBundle<M: bevy::sprite::Material2d> {
    velocity: Velocity,
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
        model: MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform {
                translation: STARTING_POSITION,
                scale: Vec3::new(100.0, 100.0, 100.0),
                ..Default::default()
            },
            material: materials.add(Color::hex("#e1c19b").unwrap()),
            ..default()
        },
    });
}
