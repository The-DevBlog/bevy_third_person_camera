use std::f32::consts::PI;

use crate::ThirdPersonCamera;
use bevy::{
    input::gamepad::{GamepadConnection::*, *},
    prelude::*,
    window::PrimaryWindow,
};

pub struct GamePadPlugin;

impl Plugin for GamePadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (connections, orbit_gamepad, zoom_gamepad));
    }
}

#[derive(Resource)]
pub struct MyGamepad {
    pub gamepad: Gamepad,
    pub x_sensitivity: f32,
    pub y_sensitivity: f32,
    pub zoom_in_button: GamepadButton,
    pub zoom_out_button: GamepadButton,
    deadzone: f32,
}

impl Default for MyGamepad {
    fn default() -> Self {
        let gamepad = Gamepad::new(0);
        MyGamepad {
            gamepad,
            x_sensitivity: 7.0,
            y_sensitivity: 4.0,
            zoom_in_button: GamepadButton::new(gamepad, GamepadButtonType::DPadUp),
            zoom_out_button: GamepadButton::new(gamepad, GamepadButtonType::DPadDown),
            deadzone: 0.5,
        }
    }
}

pub fn connections(
    mut cmds: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadConnectionEvent>,
) {
    for ev in gamepad_evr.iter() {
        match &ev.connection {
            Connected(_info) => {
                // if no gamepad is setup yet, use this one
                if my_gamepad.is_none() {
                    cmds.insert_resource(MyGamepad::default());
                }
                // println!("Gamepad connected. ID: {}, name: {}", gp.id, _info.name);
            }
            Disconnected => {
                cmds.remove_resource::<MyGamepad>();
                // println!("Gamepad disconnected: ID: {}", gp.id);
            }
        }
    }
}

pub fn zoom_gamepad(
    btns: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut cam_q: Query<&mut ThirdPersonCamera, With<ThirdPersonCamera>>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        gp.gamepad
    } else {
        return;
    };

    if let Ok(mut cam) = cam_q.get_single_mut() {
        let d_pad_down = GamepadButton::new(gamepad, GamepadButtonType::DPadDown);
        let d_pad_up = GamepadButton::new(gamepad, GamepadButtonType::DPadUp);

        // zoom out
        if btns.pressed(d_pad_down) {
            cam.radius += cam.radius * 0.01;
        // zoom in
        } else if btns.pressed(d_pad_up) {
            cam.radius -= cam.radius * 0.01;
        }
    }
}

pub fn orbit_gamepad(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&ThirdPersonCamera, &mut Transform), With<ThirdPersonCamera>>,
    axis: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    // return gamepad if one is connected
    let gamepad = if let Some(gp) = my_gamepad {
        gp
    } else {
        return;
    };

    // get X & Y axis of right joystick
    let x_axis = GamepadAxis::new(gamepad.gamepad, GamepadAxisType::RightStickX);
    let y_axis = GamepadAxis::new(gamepad.gamepad, GamepadAxisType::RightStickY);

    let mut rotation = Vec2::ZERO;
    if let (Some(x), Some(y)) = (axis.get(x_axis), axis.get(y_axis)) {
        if x.abs() > gamepad.deadzone || y.abs() > gamepad.deadzone {
            rotation = Vec2::new(x, y);
        }
    }

    for (cam, mut cam_transform) in cam_q.iter_mut() {
        if rotation.length_squared() > 0.0 {
            let window = window_q.get_single().unwrap();
            let delta_x = {
                let delta = rotation.x / window.width()
                    * std::f32::consts::PI
                    * 2.0
                    * gamepad.x_sensitivity;
                if cam.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = -rotation.y / window.height() * PI * gamepad.y_sensitivity;
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
            cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.radius));
    }
}
