use crate::scene_loader::SceneElement;
use avian3d::prelude::{ColliderConstructor, ColliderConstructorHierarchy, RigidBody};
use bevy::app::{App, Plugin, Update};
use bevy::asset::AssetServer;
use bevy::input::ButtonInput;
use bevy::prelude::{
    Children, Commands, KeyCode, On, Query, Res, ResMut, Transform, WorldAssetRoot,
};
use bevy::world_serialization::WorldInstanceReady;

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

fn spawn_fly(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let _ = &mut commands.spawn((
        WorldAssetRoot(asset_server.load("fruit_fly.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        SceneElement,
        Transform::from_xyz(
            rand::random::<f32>() * 100.0 - 50.0,
            10.0,
            rand::random::<f32>() * 100.0 - 50.0,
        ),
        RigidBody::Dynamic,
    ));
}

fn spawn_shrimp(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let _ = &mut commands.spawn((
        WorldAssetRoot(asset_server.load("pistol_shrimp.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        SceneElement,
        Transform::from_xyz(
            rand::random::<f32>() * 100.0 - 50.0,
            10.0,
            rand::random::<f32>() * 100.0 - 50.0,
        ),
        RigidBody::Dynamic,
    ));
}

fn spawn_table(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let _ = &mut commands.spawn((
        WorldAssetRoot(asset_server.load("table_set.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        RigidBody::Dynamic,
        SceneElement,
        Transform::from_xyz(
            rand::random::<f32>() * 100.0 - 50.0,
            10.0,
            rand::random::<f32>() * 100.0 - 50.0,
        ),
    ));
}
