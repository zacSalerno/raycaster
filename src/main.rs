mod debug;
mod ray;
mod ray_movement;

use bevy::prelude::*;
use debug::*;
use ray::*;
use ray_movement::*;

pub const HEIGHT: f32 = 750.0;
pub const WIDTH: f32 = 1000.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
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
        .add_plugins(RayPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
