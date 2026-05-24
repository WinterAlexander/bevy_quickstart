use bevy::app::{App, Plugin, Startup};
use bevy::camera::{Camera, Camera2d, OrthographicProjection, Projection};
use bevy::prelude::{default, Commands, Transform};
use bevy::DefaultPlugins;

pub struct Example2DPlugin;

impl Plugin for Example2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera { ..default() },
        Projection::Orthographic(OrthographicProjection::default_2d()),
        Camera2d,
        Transform::from_xyz(100.0, 200.0, 0.0),
    ));
}
