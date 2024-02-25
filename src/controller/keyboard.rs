use super::ThirdPersonController;
use bevy::prelude::*;

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement_keyboard);
    }
}

fn player_movement_keyboard(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<&mut Transform, With<ThirdPersonController>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<ThirdPersonController>)>,
) {
    for mut player_transform in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::KeyW) {
            direction += *cam.forward();
        }

        // back
        if keys.pressed(KeyCode::KeyS) {
            direction += *cam.back();
        }

        // left
        if keys.pressed(KeyCode::KeyA) {
            direction += *cam.left();
        }

        // right
        if keys.pressed(KeyCode::KeyD) {
            direction += *cam.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * 4.0 * time.delta_seconds();
        player_transform.translation += movement;

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}
