use super::ThirdPersonController;
use crate::{camera::GamepadResource, ThirdPersonCamera};
use bevy::prelude::*;

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement.run_if(movement_condition));
    }
}

fn movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut controller_q: Query<(&mut Transform, &ThirdPersonController), With<ThirdPersonController>>,
    cam_q: Query<&Transform, (With<ThirdPersonCamera>, Without<ThirdPersonController>)>,
) {
    for (mut transform, controller) in controller_q.iter_mut() {
        let cam = cam_q
            .get_single()
            .unwrap_or_else(|e| Err(format!("Error retrieving camera: {}", e)).unwrap());

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(controller.move_forward) {
            direction += *cam.forward();
        }

        // back
        if keys.pressed(controller.move_back) {
            direction += *cam.back();
        }

        // left
        if keys.pressed(controller.move_left) {
            direction += *cam.left();
        }

        // right
        if keys.pressed(controller.move_right) {
            direction += *cam.right();
        }

        // sprint
        let mut sprint = 1.0;
        if keys.pressed(controller.sprint) && controller.sprint_enabled {
            sprint = controller.sprint_speed;
        }

        direction.y = 0.0;
        let movement =
            direction.normalize_or_zero() * controller.speed * sprint * time.delta_seconds();
        transform.translation += movement;

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            transform.look_to(direction, Vec3::Y);
        }
    }
}

/// only register keyboard movement if there is no gamepad connected
fn movement_condition(gamepad: Option<Res<GamepadResource>>) -> bool {
    gamepad.is_none()
}
