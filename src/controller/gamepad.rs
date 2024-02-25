use super::ThirdPersonController;
use crate::{camera::GamepadResource, ThirdPersonCamera};
use bevy::prelude::*;

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

fn movement(
    time: Res<Time>,
    axis: Res<Axis<GamepadAxis>>,
    btns: Res<ButtonInput<GamepadButton>>,
    mut controller_q: Query<(&mut Transform, &ThirdPersonController), With<ThirdPersonController>>,
    cam_q: Query<&Transform, (With<ThirdPersonCamera>, Without<ThirdPersonController>)>,
    gamepad_res: Option<Res<GamepadResource>>,
) {
    // return gamepad if one is connected
    let gamepad = match gamepad_res {
        Some(gp) => gp.0,
        None => return,
    };

    // get X & Y axis of left joystick
    let x_axis = GamepadAxis {
        axis_type: GamepadAxisType::LeftStickX,
        gamepad,
    };
    let y_axis = GamepadAxis {
        axis_type: GamepadAxisType::LeftStickY,
        gamepad,
    };

    let mut left_joystick = Vec2::ZERO;
    if let (Some(x), Some(y)) = (axis.get(x_axis), axis.get(y_axis)) {
        left_joystick = Vec2::new(x, y);
    }

    for (mut transform, controller) in controller_q.iter_mut() {
        let cam = cam_q
            .get_single()
            .unwrap_or_else(|e| Err(format!("Error retrieving camera: {}", e)).unwrap());

        let mut direction = Vec3::ZERO;

        if left_joystick.length() > 0.5 {
            // Get the direction of the joystick relative to the camera
            let forward = cam.forward().normalize();
            let right = cam.right().normalize();
            let mut joystick_direction = forward * left_joystick.y + right * left_joystick.x;
            joystick_direction.y = 0.0;
            joystick_direction = joystick_direction.normalize();

            // Move the player in the joystick direction
            direction += joystick_direction;
        }

        // sprint
        let mut sprint = 1.0;
        if btns.pressed(controller.gamepad_settings.sprint) && controller.sprint_enabled {
            sprint = controller.sprint_speed;
        }

        direction.y = 0.0;
        transform.translation += controller.speed * sprint * direction * time.delta_seconds();

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            transform.look_to(direction, Vec3::Y);
        }
    }
}
