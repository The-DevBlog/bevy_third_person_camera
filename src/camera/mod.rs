mod gamepad;
mod mouse;
mod shared;

use crate::controller::ThirdPersonControllerPlugin;
use bevy::prelude::*;
use gamepad::GamePadPlugin;
use mouse::MousePlugin;
use shared::SharedPlugin;

/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_third_person_camera::camera::*;
/// fn main() {
///     App::new().add_plugins(ThirdPersonCameraPlugin);
/// }
/// ```
pub struct ThirdPersonCameraPlugin;

impl Plugin for ThirdPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MousePlugin,
            GamePadPlugin,
            SharedPlugin,
            ThirdPersonControllerPlugin,
        ));
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
    pub gamepad_settings: CustomGamepadSettings,
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
            gamepad_settings: CustomGamepadSettings::default(),
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

/// Sets the zoom bounds (min & max)
pub struct Zoom {
    pub min: f32,
    pub max: f32,
    radius: f32,
    radius_copy: Option<f32>,
}

impl Zoom {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            radius: (min + max) / 2.0,
            radius_copy: None,
        }
    }
}

/// Offset the camera behind the player. For example, an offset value of (0.5, 0.25) will
/// place the camera closer the player's right shoulder
pub struct Offset {
    pub offset: (f32, f32),
    offset_copy: (f32, f32),
    is_transitioning: bool,
}

impl Offset {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            offset: (x, y),
            offset_copy: (x, y),
            is_transitioning: false,
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
/// use bevy_third_person_camera::camera::*;
/// fn spawn_camera(mut commands: Commands) {
///    let gamepad = Gamepad::new(0);
///    commands.spawn((
///        ThirdPersonCamera {
///            gamepad_settings: CustomGamepadSettings {
///                aim_button: GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2),
///                mouse_orbit_button: GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger),
///                offset_toggle_button: GamepadButton::new(gamepad, GamepadButtonType::DPadRight),
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
    pub aim_button: GamepadButton,
    pub mouse_orbit_button: GamepadButton,
    pub offset_toggle_button: GamepadButton,
    pub x_sensitivity: f32,
    pub y_sensitivity: f32,
    pub zoom_in_button: GamepadButton,
    pub zoom_out_button: GamepadButton,
}

impl Default for CustomGamepadSettings {
    fn default() -> Self {
        let gamepad = Gamepad::new(0);
        Self {
            aim_button: GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2),
            mouse_orbit_button: GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger),
            offset_toggle_button: GamepadButton::new(gamepad, GamepadButtonType::DPadRight),
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
