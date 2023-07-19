mod gamepad;
mod mouse;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use gamepad::GamePadPlugin;
use mouse::MousePlugin;

pub struct ThirdPersonCameraPlugin;

impl Plugin for ThirdPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MousePlugin, GamePadPlugin))
            .add_systems(PostStartup, toggle_cursor)
            .add_systems(
                Update,
                (
                    sync_player_camera,
                    toggle_cursor.run_if(toggle_cursor_enabled),
                ),
            );
    }
}

#[derive(Component)]
pub struct ThirdPersonCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub mouse_sensitivity: f32,
    pub enable_cursor_lock_toggle: bool,
    pub lock_cursor: bool,
    pub cursor_lock_key: KeyCode,
    pub zoom_bounds: (f32, f32),
    pub gamepad_settings: GamepadSettings,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        ThirdPersonCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            mouse_sensitivity: 1.0,
            enable_cursor_lock_toggle: true,
            lock_cursor: true,
            cursor_lock_key: KeyCode::Space,
            zoom_bounds: (5.0, 10.0),
            gamepad_settings: GamepadSettings::default(),
        }
    }
}

#[derive(Resource)]
pub struct GamepadResource(pub Gamepad);

#[derive(Component)]
pub struct GamepadSettings {
    pub x_sensitivity: f32,
    pub y_sensitivity: f32,
    pub zoom_in_button: GamepadButton,
    pub zoom_out_button: GamepadButton,
    deadzone: f32,
}

impl Default for GamepadSettings {
    fn default() -> Self {
        let gamepad = Gamepad::new(0);
        Self {
            x_sensitivity: 7.0,
            y_sensitivity: 4.0,
            zoom_in_button: GamepadButton::new(gamepad, GamepadButtonType::DPadUp),
            zoom_out_button: GamepadButton::new(gamepad, GamepadButtonType::DPadDown),
            deadzone: 0.5,
        }
    }
}

#[derive(Component)]
pub struct ThirdPersonCameraTarget;

fn sync_player_camera(
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

fn toggle_cursor(
    mut cam_q: Query<&mut ThirdPersonCamera>,
    keys: Res<Input<KeyCode>>,
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut cam) = cam_q.get_single_mut() else { return };

    if keys.just_pressed(cam.cursor_lock_key) {
        cam.lock_cursor = !cam.lock_cursor;
    }

    let mut window = window_q.get_single_mut().unwrap();
    if cam.lock_cursor {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    } else {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

/// checks if the toggle cursor functionality is enabled
fn toggle_cursor_enabled(cam_q: Query<&ThirdPersonCamera>) -> bool {
    let Ok(cam) = cam_q.get_single() else { return true };
    cam.enable_cursor_lock_toggle
}
