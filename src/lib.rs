mod gamepad;
mod mouse;

use bevy::prelude::*;
use gamepad::GamePadPlugin;
use mouse::MousePlugin;

pub struct ThirdPersonCameraPlugin;

impl Plugin for ThirdPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MousePlugin, GamePadPlugin))
            .add_systems(Update, sync_player_camera);
    }
}

#[derive(Component)]
pub struct ThirdPersonCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub mouse_sensitivity: f32,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        ThirdPersonCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
            mouse_sensitivity: 1.0,
        }
    }
}

#[derive(Component)]
pub struct ThirdPersonCameraTarget;

pub fn sync_player_camera(
    player_q: Query<&Transform, With<ThirdPersonCameraTarget>>,
    mut cam_q: Query<(&mut ThirdPersonCamera, &mut Transform), Without<ThirdPersonCameraTarget>>,
) {
    let Ok(player) = player_q.get_single() else { return };
    let Ok((mut camera, mut camera_transform)) = cam_q.get_single_mut() else { return };

    let delta = player.translation - camera.focus;

    if delta != Vec3::ZERO {
        camera.focus = player.translation;
        camera_transform.translation += delta;
    }
}
