use crate::title_screen::GameState;
use avian3d::parry::glamx::{EulerRot, Quat, Vec3};
use bevy::app::{App, Plugin, Update};
use bevy::camera::Camera3d;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::{
    Component, MessageReader, OnEnter, OnExit, Query, Res, ResMut, Resource, Single, Time,
    Transform, With, Without,
};

pub struct ThirdPersonCameraPlugin;

impl Plugin for ThirdPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (process_inputs, update_state, set_camera));
        app.add_systems(
            OnEnter(GameState::Game),
            |mut c: ResMut<CameraControllerState>| c.enabled = true,
        );
        app.add_systems(
            OnExit(GameState::Game),
            |mut c: ResMut<CameraControllerState>| c.enabled = false,
        );
        app.insert_resource(CameraControllerState::default());
    }
}

#[derive(Component)]
pub struct CameraTarget;

#[derive(Resource)]
struct CameraControllerState {
    unprocessed_delta: Option<(f32, f32)>,

    pitch: f32,
    yaw: f32,
    distance: f32,

    enabled: bool,
}

impl Default for CameraControllerState {
    fn default() -> Self {
        CameraControllerState {
            unprocessed_delta: None,
            pitch: -45.0f32,
            yaw: 0.0,
            distance: 5.0,
            enabled: true,
        }
    }
}

impl CameraControllerState {
    fn to_quat(&self) -> Quat {
        Quat::from_euler(EulerRot::XYZ, 0.0, self.yaw, 0.0)
            * Quat::from_euler(EulerRot::XYZ, self.pitch, 0.0, 0.0)
    }
    fn to_vector(&self) -> Vec3 {
        let horizontal_scale = self.pitch.cos();
        Vec3::new(
            (self.yaw + std::f32::consts::PI).sin() * horizontal_scale,
            self.pitch.sin(),
            (self.yaw + std::f32::consts::PI).cos() * horizontal_scale,
        )
        .normalize()
    }
}

fn process_inputs(
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mut keyboard_input_events: MessageReader<KeyboardInput>,
    mut state: ResMut<CameraControllerState>,
) {
    for event in mouse_motion_events.read() {
        state.unprocessed_delta = match state.unprocessed_delta {
            Some((x, y)) => Some((x + event.delta.x, y + event.delta.y)),
            None => Some((event.delta.x, event.delta.y)),
        };
    }
}

fn update_state(mut state: ResMut<CameraControllerState>, time: Res<Time>) {
    if !state.enabled {
        return;
    }

    if let Some(unprocessed_delta) = state.unprocessed_delta {
        let mouse_sensitivity = 0.002;

        state.yaw += -unprocessed_delta.0 * mouse_sensitivity;
        state.pitch = (state.pitch + (-unprocessed_delta.1 * mouse_sensitivity))
            .clamp((-90.0f32).to_radians(), (90.0f32).to_radians());
    }
    state.unprocessed_delta = None;
}

fn set_camera(
    camera_target_query: Single<&Transform, (With<CameraTarget>, Without<Camera3d>)>,
    mut camera_transform_query: Query<&mut Transform, With<Camera3d>>,
    state: Res<CameraControllerState>,
) {
    let mut camera_transform = camera_transform_query.single_mut().unwrap();

    if !state.enabled {
        return;
    }

    camera_transform.rotation = state.to_quat();
    camera_transform.translation =
        camera_target_query.translation - state.to_vector() * state.distance;
}
