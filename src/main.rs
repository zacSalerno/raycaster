use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, circle_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn circle_movement() {}
