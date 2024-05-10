mod constants;
mod debug;
mod ray;
mod shape;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use constants::*;
use debug::*;
use ray::*;
use shape::*;

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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(RayPlugin)
        .add_plugins(ShapePlugin)
        .add_plugins(DebugPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
