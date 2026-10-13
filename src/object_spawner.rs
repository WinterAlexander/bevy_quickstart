use crate::scene_loader::SceneElement;
use avian3d::prelude::{ColliderConstructor, ColliderConstructorHierarchy, RigidBody};
use bevy::app::{App, Plugin, Update};
use bevy::asset::AssetServer;
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, EntityCommands, KeyCode, Res, ResMut, Transform, WorldAssetRoot};

pub struct ObjectSpawnerPlugin;

impl Plugin for ObjectSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, object_spawner);
    }
}

fn object_spawner(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut asset_server: ResMut<AssetServer>,
) {
    if input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight) {
        return;
    }

    if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::Digit1) {
        spawn_fly(commands, asset_server);
    } else if input.just_pressed(KeyCode::Numpad2) || input.just_pressed(KeyCode::Digit2) {
        spawn_shrimp(commands, asset_server);
    } else if input.just_pressed(KeyCode::Numpad3) || input.just_pressed(KeyCode::Digit3) {
        spawn_table(commands, asset_server);
    }
}

fn spawn_fly(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
    add_object_tag(&mut commands.spawn((
        WorldAssetRoot(asset_server.load("fruit_fly.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
    )));
}

fn spawn_shrimp(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
    add_object_tag(&mut commands.spawn((
        WorldAssetRoot(asset_server.load("pistol_shrimp.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
    )));
}

fn spawn_table(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
    add_object_tag(&mut commands.spawn((
        WorldAssetRoot(asset_server.load("table_set.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
    )));
}

fn add_object_tag(commands: &mut EntityCommands) {
    commands
        .insert(SceneElement)
        .insert(Transform::from_xyz(
            rand::random::<f32>() * 100.0 - 50.0,
            10.0,
            rand::random::<f32>() * 100.0 - 50.0,
        ))
        .insert(RigidBody::Dynamic);
}
