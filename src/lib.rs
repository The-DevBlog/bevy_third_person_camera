pub mod camera;
pub mod controller;

use bevy::prelude::*;
use camera::{CameraGamepadSettings, CameraPlugin, Offset, Zoom};

pub struct ThirdPersonCameraPlugin;

impl Plugin for ThirdPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlugin);
    }
}

/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_third_person_camera::camera::*
/// fn spawn_camera(mut commands: Commands) {
///     commands.spawn((
///         ThirdPersonCamera::default(),
///         Camera3dBundle::default()
///     ));
/// }
/// ```
#[derive(Component)]
pub struct ThirdPersonCamera {
    pub aim_enabled: bool,
    pub aim_button: MouseButton,
    pub aim_speed: f32,
    pub aim_zoom: f32,
    pub cursor_lock_toggle_enabled: bool,
    pub cursor_lock_active: bool,
    pub cursor_lock_key: KeyCode,
    pub gamepad_settings: CameraGamepadSettings,
    pub mouse_sensitivity: f32,
    pub mouse_orbit_button_enabled: bool,
    pub mouse_orbit_button: MouseButton,
    pub offset_enabled: bool,
    pub offset: Offset,
    pub offset_toggle_enabled: bool,
    pub offset_toggle_key: KeyCode,
    pub offset_toggle_speed: f32,
    pub zoom_enabled: bool,
    pub zoom: Zoom,
    pub zoom_sensitivity: f32,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        ThirdPersonCamera {
            aim_enabled: false,
            aim_button: MouseButton::Right,
            aim_speed: 3.0,
            aim_zoom: 0.7,
            cursor_lock_key: KeyCode::Space,
            cursor_lock_toggle_enabled: true,
            gamepad_settings: CameraGamepadSettings::default(),
            cursor_lock_active: true,
            mouse_sensitivity: 1.0,
            mouse_orbit_button_enabled: false,
            mouse_orbit_button: MouseButton::Middle,
            offset_enabled: false,
            offset: Offset::new(0.5, 0.4),
            offset_toggle_enabled: false,
            offset_toggle_speed: 5.0,
            offset_toggle_key: KeyCode::KeyE,
            zoom_enabled: true,
            zoom: Zoom::new(1.5, 3.0),
            zoom_sensitivity: 1.0,
        }
    }
}

/// The desired target for the third person camera to look at
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_third_person_camera::camera::*;
/// fn spawn_player(mut commands: Commands) {
///     commands.spawn((
///         PbrBundle::default(),
///         ThirdPersonCameraTarget
///     ));
/// }
/// ```
#[derive(Component)]
pub struct ThirdPersonCameraTarget;
