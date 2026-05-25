use bevy::app::{App, Plugin, Update};
use bevy::camera::Camera;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::input::ButtonInput;
use bevy::prelude::{GlobalTransform, MouseButton, Query, Res, Transform};

pub struct TopDownCameraControllerPlugin;

const DRAG_BUTTON: MouseButton = MouseButton::Left;
const MOUSE_SENSITIVITY: f32 = 1.0;
const SCROLL_SENSITIVITY: f32 = 0.01;

impl Plugin for TopDownCameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera);
    }
}

fn update_camera(
    mut camera_transform_query: Query<(&Camera, &GlobalTransform, &mut Transform)>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    let (camera, gt, mut transform) = camera_transform_query.single_mut().unwrap();
    if mouse_button_input.pressed(DRAG_BUTTON) {
        let position = camera.world_to_viewport(gt, transform.translation).unwrap();
        let world_pos = camera
            .viewport_to_world_2d(gt, position - mouse_motion.delta * MOUSE_SENSITIVITY)
            .unwrap();
        transform.translation = world_pos.extend(0.0);
    }

    transform.scale += mouse_scroll.delta.y * SCROLL_SENSITIVITY;
}
