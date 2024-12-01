use std::f32::consts::PI;

use crate::{zoom_condition, ThirdPersonCamera};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct GamePadPlugin;

impl Plugin for GamePadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GamepadConnected>()
            .add_systems(
                PreUpdate,
                orbit_gamepad.run_if(resource_equals(GamepadConnected(true))),
            )
            .add_systems(
                Update,
                (
                    detect_gamepad,
                    zoom_gamepad
                        .run_if(resource_equals(GamepadConnected(true)))
                        .run_if(zoom_condition),
                ),
            );
    }
}

#[derive(Resource, PartialEq, Default)]
struct GamepadConnected(pub bool);

fn detect_gamepad(mut gamepad_connected: ResMut<GamepadConnected>, gamepad_q: Query<&Gamepad>) {
    gamepad_connected.0 = !gamepad_q.is_empty();
}

pub fn zoom_gamepad(
    btns: Query<&Gamepad>,
    mut cam_q: Query<&mut ThirdPersonCamera, With<ThirdPersonCamera>>,
) {
    if let Ok(mut cam) = cam_q.get_single_mut() {
        let gp = &cam.gamepad_settings;

        let zoom_out = gp.zoom_out_button;
        let zoom_in = gp.zoom_in_button;

        let mut new_radius = cam.zoom.radius;

        // zoom out
        for btns in btns.iter() {
            if btns.pressed(zoom_out) {
                new_radius += cam.zoom.radius * 0.01;
                cam.zoom.radius = new_radius.clamp(cam.zoom.min, cam.zoom.max);
            // zoom in
            } else if btns.pressed(zoom_in) {
                new_radius -= cam.zoom.radius * 0.01;
                cam.zoom.radius = new_radius.clamp(cam.zoom.min, cam.zoom.max);
            }
        }
    }
}

pub fn orbit_gamepad(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&ThirdPersonCamera, &mut Transform), With<ThirdPersonCamera>>,
    gamepad_q: Query<&Gamepad>,
) {
    let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else {
        return;
    };

    let Ok(gamepad) = gamepad_q.get_single() else {
        return;
    };

    if cam.mouse_orbit_button_enabled && !gamepad.pressed(cam.gamepad_settings.mouse_orbit_button) {
        return;
    }

    let x_axis = gamepad.right_stick().x;
    let y_axis = gamepad.right_stick().y;

    let deadzone = 0.5;
    let mut rotation = Vec2::ZERO;
    let (x, y) = (x_axis, y_axis);
    if x.abs() > deadzone || y.abs() > deadzone {
        rotation = Vec2::new(x, y);
    }

    if rotation.length_squared() > 0.0 {
        let window = window_q.get_single().unwrap();
        let delta_x = {
            let delta = rotation.x / window.width()
                * std::f32::consts::PI
                * 2.0
                * cam.gamepad_settings.sensitivity.x;
            delta
        };
        let delta_y = -rotation.y / window.height() * PI * cam.gamepad_settings.sensitivity.y;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        cam_transform.rotation = yaw * cam_transform.rotation; // rotate around global y axis

        let new_rotation = cam_transform.rotation * pitch;

        // check if new rotation will cause camera to go beyond the 180 degree vertical bounds
        let up_vector = new_rotation * Vec3::Y;
        if up_vector.y > 0.0 {
            cam_transform.rotation = new_rotation;
        }
    }

    let rot_matrix = Mat3::from_quat(cam_transform.rotation);
    cam_transform.translation = rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.zoom.radius));
}
