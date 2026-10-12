use crate::mesh_loader::{GLTFLoadConfig, MeshLoader, load_gltf};
use crate::scene_loader::SceneElement;
use avian3d::prelude::{Collider, MassPropertiesBundle, RigidBody};
use bevy::app::{App, Plugin, Update};
use bevy::asset::AssetServer;
use bevy::input::ButtonInput;
use bevy::prelude::{EntityCommands, KeyCode, Res, ResMut, Transform};
use bevy::utils::default;

pub struct ObjectSpawnerPlugin;

impl Plugin for ObjectSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, object_spawner);
    }
}

fn object_spawner(
    input: Res<ButtonInput<KeyCode>>,
    mut asset_server: ResMut<AssetServer>,
    mut mesh_loader: ResMut<MeshLoader>,
) {
    if input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight) {
        return;
    }

    if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::Digit1) {
        spawn_fly(asset_server, mesh_loader);
    } else if input.just_pressed(KeyCode::Numpad2) || input.just_pressed(KeyCode::Digit2) {
        spawn_shrimp(asset_server, mesh_loader);
    } else if input.just_pressed(KeyCode::Numpad3) || input.just_pressed(KeyCode::Digit3) {
        spawn_table(asset_server, mesh_loader);
    }
}

fn spawn_fly(mut asset_server: ResMut<AssetServer>, mut mesh_loader: ResMut<MeshLoader>) {
    load_gltf(
        String::from("fruit_fly.glb"),
        GLTFLoadConfig {
            entity_initializer: add_object_tag,
            collider_body: Some(RigidBody::Dynamic),
            ..default()
        },
        &mut asset_server,
        &mut mesh_loader,
    );
}

fn spawn_shrimp(mut asset_server: ResMut<AssetServer>, mut mesh_loader: ResMut<MeshLoader>) {
    load_gltf(
        String::from("pistol_shrimp.glb"),
        GLTFLoadConfig {
            entity_initializer: add_object_tag,
            collider_body: Some(RigidBody::Dynamic),
            ..default()
        },
        &mut asset_server,
        &mut mesh_loader,
    );
}

fn spawn_table(mut asset_server: ResMut<AssetServer>, mut mesh_loader: ResMut<MeshLoader>) {
    load_gltf(
        String::from("table_set.glb"),
        GLTFLoadConfig {
            entity_initializer: add_object_tag,
            collider_body: None,
            ..default()
        },
        &mut asset_server,
        &mut mesh_loader,
    );
}

fn add_object_tag(commands: &mut EntityCommands) {
    commands
        .insert(SceneElement)
        .insert(Transform::from_xyz(
            rand::random::<f32>() * 100.0 - 50.0,
            10.0,
            rand::random::<f32>() * 100.0 - 50.0,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.0, 1.0, 1.0));
}
