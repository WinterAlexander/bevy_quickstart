use bevy::app::{App, Plugin, Startup};
use bevy::asset::Assets;
use bevy::camera::{Camera, Camera2d, OrthographicProjection, Projection};
use bevy::color::Color;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{
    default, ColorMaterial, Commands, MeshMaterial2d, Rectangle, ResMut, Transform,
};

pub struct Example2DPlugin;

impl Plugin for Example2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, spawn_object));
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

fn spawn_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(10.0, 10.0));
    let material = materials.add(Color::srgb(1.0, 0.0, 0.0));
    commands.spawn((
        Transform::from_xyz(50.0, 50.0, 0.0),
        Mesh2d(mesh),
        MeshMaterial2d(material),
    ));
}
