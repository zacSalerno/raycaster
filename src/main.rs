use bevy::{
    input::*,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub const HEIGHT: f32 = 750.0;
pub const WIDTH: f32 = 1000.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WIDTH, HEIGHT).into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, circle_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let circle = Mesh2dHandle(meshes.add(Circle { radius: 50.0 }));

    let color = Color::rgb(255.0, 0.0, 0.0);

    commands.spawn(MaterialMesh2dBundle {
        mesh: circle,
        material: materials.add(color),
        transform: Transform::from_xyz(200.0, 0.0, 0.0),
        ..default()
    });
}

fn circle_movement(
    mut balls: Query<(&mut Transform, &Mesh2dHandle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut balls {
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
    }
}
