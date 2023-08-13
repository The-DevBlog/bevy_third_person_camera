mod gamepad;
mod mouse;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use gamepad::{orbit_gamepad, GamePadPlugin};
use mouse::{orbit_mouse, MousePlugin};

/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_third_person_camera::ThirdPersonCameraPlugin;
/// fn main() {
///     App::new().add_plugins(ThirdPersonCameraPlugin);
/// }
/// ```
pub struct ThirdPersonCameraPlugin;

impl Plugin for ThirdPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MousePlugin, GamePadPlugin)).add_systems(
            Update,
            (
                sync_player_camera.after(orbit_mouse).after(orbit_gamepad),
                toggle_cursor.run_if(toggle_cursor_enabled),
            ),
        );
    }
}

/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_third_person_camera::ThirdPersonCamera;
/// fn spawn_camera(mut commands: Commands) {
///     commands.spawn((
///         ThirdPersonCamera::default(),
///         Camera3dBundle::default()
///     ));
/// }
/// ```
#[derive(Component)]
pub struct ThirdPersonCamera {
    pub cursor_lock_key: KeyCode,
    pub enable_cursor_lock_toggle: bool,
    pub focus: Vec3,
    pub gamepad_settings: CustomGamepadSettings,
    pub lock_cursor: bool,
    pub mouse_sensitivity: f32,
    pub radius: f32,
    pub xy_offset: (f32, f32),
    pub zoom_bounds: (f32, f32),
    pub zoom_sensitivity: f32,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        ThirdPersonCamera {
            cursor_lock_key: KeyCode::Space,
            enable_cursor_lock_toggle: true,
            focus: Vec3::ZERO,
            gamepad_settings: CustomGamepadSettings::default(),
            lock_cursor: true,
            mouse_sensitivity: 1.0,
            radius: 5.0,
            xy_offset: (0.0, 0.0),
            zoom_bounds: (3.0, 10.0),
            zoom_sensitivity: 1.0,
        }
    }
}

#[derive(Resource)]
pub struct GamepadResource(pub Gamepad);

/// Customizable gamepad settings
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_third_person_camera::{CustomGamepadSettings, ThirdPersonCamera};
/// fn spawn_camera(mut commands: Commands) {
///    let gamepad = Gamepad::new(0);
///    commands.spawn((
///        ThirdPersonCamera {
///            gamepad_settings: CustomGamepadSettings {
///                x_sensitivity: 7.0,
///                y_sensitivity: 4.0,
///                zoom_in_button: GamepadButton::new(gamepad, GamepadButtonType::DPadUp),
///                zoom_out_button: GamepadButton::new(gamepad, GamepadButtonType::DPadDown),
///            },
///            ..default()
///        },
///        Camera3dBundle::default(),
///    ));
/// }
/// ```
#[derive(Component)]
pub struct CustomGamepadSettings {
    pub x_sensitivity: f32,
    pub y_sensitivity: f32,
    pub zoom_in_button: GamepadButton,
    pub zoom_out_button: GamepadButton,
}

impl Default for CustomGamepadSettings {
    fn default() -> Self {
        let gamepad = Gamepad::new(0);
        Self {
            x_sensitivity: 7.0,
            y_sensitivity: 4.0,
            zoom_in_button: GamepadButton::new(gamepad, GamepadButtonType::DPadUp),
            zoom_out_button: GamepadButton::new(gamepad, GamepadButtonType::DPadDown),
        }
    }
}

/// The desired target for the third person camera to look at
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_third_person_camera::ThirdPersonCameraTarget;
/// fn spawn_player(mut commands: Commands) {
///     commands.spawn((
///         PbrBundle::default(),
///         ThirdPersonCameraTarget
///     ));
/// }
/// ```
#[derive(Component)]
pub struct ThirdPersonCameraTarget;

fn sync_player_camera(
    player_q: Query<&Transform, With<ThirdPersonCameraTarget>>,
    mut cam_q: Query<(&mut ThirdPersonCamera, &mut Transform), Without<ThirdPersonCameraTarget>>,
) {
    let Ok(player) = player_q.get_single() else { return };
    let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else { return };

    // Calculate the desired camera translation based on focus, radius, and xy_offset
    let rotation_matrix = Mat3::from_quat(cam_transform.rotation);
    let offset = rotation_matrix.mul_vec3(Vec3::new(cam.xy_offset.0, cam.xy_offset.1, 0.0));
    let desired_translation =
        cam.focus + rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.radius)) + offset;

    // Update the camera translation and focus
    let delta = player.translation - cam.focus;
    cam_transform.translation = desired_translation + delta;
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
