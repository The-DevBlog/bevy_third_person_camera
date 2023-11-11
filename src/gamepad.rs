use std::f32::consts::PI;

use crate::{GamepadResource, ThirdPersonCamera};
use bevy::{
    input::gamepad::{GamepadConnection::*, *},
    prelude::*,
    window::PrimaryWindow,
};

pub struct GamePadPlugin;

impl Plugin for GamePadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                connections,
                (orbit_gamepad, zoom_gamepad).run_if(resource_exists::<GamepadResource>()),
            ),
        );
    }
}

fn connections(
    mut cmds: Commands,
    gamepad_res: Option<Res<GamepadResource>>,
    mut gamepad_evr: EventReader<GamepadConnectionEvent>,
) {
    for ev in gamepad_evr.read() {
        match &ev.connection {
            Connected(_info) => {
                // if no gamepad is setup yet, use this one
                if gamepad_res.is_none() {
                    cmds.insert_resource(GamepadResource(Gamepad::new(0)));
                }
                // println!("Gamepad connected");
            }
            Disconnected => {
                cmds.remove_resource::<GamepadResource>();
                // println!("Gamepad disconnected");
            }
        }
    }
}

pub fn zoom_gamepad(
    btns: Res<Input<GamepadButton>>,
    gamepad_res: Option<Res<GamepadResource>>,
    mut cam_q: Query<&mut ThirdPersonCamera, With<ThirdPersonCamera>>,
) {
    let gamepad = if let Some(gp) = gamepad_res {
        gp.0
    } else {
        return;
    };

    if let Ok(mut cam) = cam_q.get_single_mut() {
        let gp = &cam.gamepad_settings;

        let zoom_out = GamepadButton::new(gamepad, gp.zoom_out_button.button_type);
        let zoom_in = GamepadButton::new(gamepad, gp.zoom_in_button.button_type);

        let mut new_radius = cam.zoom.radius;

        // zoom out
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

pub fn orbit_gamepad(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&ThirdPersonCamera, &mut Transform), With<ThirdPersonCamera>>,
    btns: Res<Input<GamepadButton>>,
    axis: Res<Axis<GamepadAxis>>,
    gamepad_res: Option<Res<GamepadResource>>,
) {
    // return gamepad if one is connected
    let gamepad = if let Some(gp) = gamepad_res {
        gp.0
    } else {
        return;
    };

    let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else { return };

    if cam.mouse_orbit_button_enabled && !btns.pressed(cam.gamepad_settings.mouse_orbit_button) {
        return;
    }

    let x_axis = GamepadAxis::new(gamepad, GamepadAxisType::RightStickX);
    let y_axis = GamepadAxis::new(gamepad, GamepadAxisType::RightStickY);

    let deadzone = 0.5;
    let mut rotation = Vec2::ZERO;
    if let (Some(x), Some(y)) = (axis.get(x_axis), axis.get(y_axis)) {
        if x.abs() > deadzone || y.abs() > deadzone {
            rotation = Vec2::new(x, y);
        }
    }

    if rotation.length_squared() > 0.0 {
        let window = window_q.get_single().unwrap();
        let delta_x = {
            let delta = rotation.x / window.width()
                * std::f32::consts::PI
                * 2.0
                * cam.gamepad_settings.x_sensitivity;
            delta
        };
        let delta_y = -rotation.y / window.height() * PI * cam.gamepad_settings.y_sensitivity;
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
    cam_transform.translation =
        cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.zoom.radius));
}
