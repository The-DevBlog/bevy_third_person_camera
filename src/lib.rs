mod gamepad;
mod mouse;

use bevy::{
    input::gamepad::GamepadConnection, prelude::*, window::{CursorGrabMode, PrimaryWindow}
};
use gamepad::GamePadPlugin;
use mouse::MousePlugin;

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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSyncSet;

impl Plugin for ThirdPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MousePlugin, GamePadPlugin))
            .add_systems(
                Update,
                (
                    aim.run_if(aim_condition),
                    toggle_x_offset.run_if(toggle_x_offset_condition),
                    toggle_cursor.run_if(toggle_cursor_condition),
                ),
            )
            .add_systems(
                PostUpdate,
                sync_player_camera
                    .before(TransformSystem::TransformPropagate)
                    .in_set(CameraSyncSet),
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
    /// Flag to indicate if the aim functionality is turned on.
    /// Default is false
    pub aim_enabled: bool,
    /// The mouse aim button binding.
    /// Default is MouseButton::Right
    pub aim_button: MouseButton,
    /// The speed at which aiming occurs.
    /// Default is 3.0
    pub aim_speed: f32,
    /// The zoom level of the aim zooming functionality.
    /// This value should be >= 0.l, and <= Zoom.min.
    /// The smaller the value, the greater the zoom distance. 0.1 would essentially look like 'first person'.
    /// Default is 0.7
    pub aim_zoom: f32,
    /// Flag to indicate if the cursor lock toggle functionality is turned on.
    /// When enabled and the cursor lock is NOT active, the mouse can freely move about the window without the camera's transform changing.
    /// Example usage: Browsing a character inventory without moving the camera.
    /// Default is true
    pub cursor_lock_toggle_enabled: bool,
    /// Flag to indicate if the cursor is in a locked state or not.
    /// Default is true
    pub cursor_lock_active: bool,
    /// The cursor lock toggle key binding.
    /// Default is KeyCode::Space
    pub cursor_lock_key: KeyCode,
    /// Custom gamepad settings.
    pub gamepad_settings: CustomGamepadSettings,
    /// Mouse x/y sensitivity
    /// Default is Vec2::new(1.0, 1.0)
    pub sensitivity: Vec2,
    /// Flag to indicate if the orbiting functionality is controlled when a mouse button is being pressed.
    /// Default is false
    pub mouse_orbit_button_enabled: bool,
    /// The mouse button binding to control when the orbiting occurs.
    /// Default is MouseButton:Middle
    pub mouse_orbit_button: MouseButton,
    /// Flag to indicate whether there is a camera offset applied or not.
    /// Default is false
    pub offset_enabled: bool,
    /// The camera offset relative to the camera target.
    /// Offset is a tuple. The first value corresponds to the x offset, the second value to the y offset.
    /// Example: offset: Offset::new(<my_x_value>, <my_y_value>);
    /// Default is Offset::new(0.5, 0.4)
    pub offset: Offset,
    /// Inverts the x value of the offset.
    /// Example: If the x offset is set to 5.0, then the x offset will be inverted to -5.0 if this is set to true.
    /// Default is false
    pub offset_toggle_enabled: bool,
    /// The key binding of the offset toggle
    /// Default is KeyCode::KeyE
    pub offset_toggle_key: KeyCode,
    /// The speed at which the x offset will transition.
    /// Default is 5.0
    pub offset_toggle_speed: f32,
    /// Flag to indicate whether a camera zoom is applied or not.
    /// Default is true
    pub zoom_enabled: bool,
    /// The min/max bounds of the camera.
    /// This is different than the aim zoom. This zoom is the position the camera stays in relation to the camera target.
    /// Zoom is a tuple. The first value is the minimum zoom. The smaller the value, the closer the camera can come to it's target. The second value is the maximum zoom. The greater the maximum zoom, the farther away the camera can go from it's target.
    /// The zoom is adjusted using the scroll wheel.
    /// Default is Zoom::new(1.5, 3.0);
    pub zoom: Zoom,
    /// The speed at which the scroll wheel zooms in or out.
    /// Default is 1.0
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
            sensitivity: Vec2::new(1.0, 1.0),
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
pub struct GamepadResource(pub GamepadConnection);

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
///                aim_button: GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2),
///                mouse_orbit_button: GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger),
///                offset_toggle_button: GamepadButton::new(gamepad, GamepadButtonType::DPadRight),
///                sensitivity: Vec2::new(7.0, 4.0),
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
    /// The aim button binding.
    /// Default is GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2)
    pub aim_button: GamepadButton,
    /// The button binding to control when the orbiting occurs.
    /// Default is GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger)
    pub mouse_orbit_button: GamepadButton,
    /// The button binding of the offset toggle
    /// Default is GamepadButton::new(gamepad, GamepadButtonType::DPadRight)
    pub offset_toggle_button: GamepadButton,
    /// Gamepad x/y sensitivity
    /// Default is Vec2::new(7.0, 4.0)
    pub sensitivity: Vec2,
    /// The camera zoom in button binding (equivalent to the scroll wheel zoom)
    /// GamepadButton::new(gamepad, GamepadButtonType::DPadUp)
    pub zoom_in_button: GamepadButton,
    /// The camera zoom out button binding (equivalent to the scroll wheel zoom)
    /// GamepadButton::new(gamepad, GamepadButtonType::DPadDown)
    pub zoom_out_button: GamepadButton,
}

impl Default for CustomGamepadSettings {
    fn default() -> Self {
        Self {
            aim_button: GamepadButton::LeftTrigger2,
            mouse_orbit_button: GamepadButton::LeftTrigger,
            offset_toggle_button: GamepadButton::DPadRight,
            sensitivity: Vec2::new(7.0, 4.0),
            zoom_in_button: GamepadButton::DPadUp,
            zoom_out_button: GamepadButton::DPadDown,
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
    let Ok(player) = player_q.get_single() else {
        return;
    };
    let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else {
        return;
    };

    // Calculate the desired camera translation based, radius, and xy_offset
    let rotation_matrix = Mat3::from_quat(cam_transform.rotation);

    // apply the offset if offset_enabled is true
    let mut offset = Vec3::ZERO;
    if cam.offset_enabled {
        offset = rotation_matrix.mul_vec3(Vec3::new(cam.offset.offset.0, cam.offset.offset.1, 0.0));
    }

    let desired_translation =
        rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.zoom.radius)) + offset;

    // Update the camera translation
    cam_transform.translation = desired_translation + player.translation;
}

// only run aiming logic if `aim_enabled` is true
fn aim_condition(cam_q: Query<&ThirdPersonCamera, With<ThirdPersonCamera>>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return false;
    };
    cam.aim_enabled
}

fn aim(
    mut cam_q: Query<
        (&mut ThirdPersonCamera, &Transform),
        (With<ThirdPersonCamera>, Without<ThirdPersonCameraTarget>),
    >,
    mouse: Res<ButtonInput<MouseButton>>,
    mut player_q: Query<&mut Transform, With<ThirdPersonCameraTarget>>,
    btns: Res<ButtonInput<GamepadButton>>,
    time: Res<Time>,
) {
    let Ok((mut cam, cam_transform)) = cam_q.get_single_mut() else {
        return;
    };

    // check if aim button was pressed
    let aim_btn = mouse.pressed(cam.aim_button) || btns.pressed(cam.gamepad_settings.aim_button);

    if aim_btn {
        // rotate player or target to face direction he is aiming
        let Ok(mut player_transform) = player_q.get_single_mut() else {
            return;
        };
        player_transform.look_to(*cam_transform.forward(), Vec3::Y);

        let desired_zoom = cam.zoom.min * cam.aim_zoom;

        // radius_copy is used for restoring the radius (zoom) to it's
        // original value after releasing the aim button
        if cam.zoom.radius_copy.is_none() {
            cam.zoom.radius_copy = Some(cam.zoom.radius);
        }

        let zoom_factor =
            (cam.zoom.radius_copy.unwrap() / cam.aim_zoom) * cam.aim_speed * time.delta_secs();

        // stop zooming in if current radius is less than desired zoom
        if cam.zoom.radius <= desired_zoom || cam.zoom.radius - zoom_factor <= desired_zoom {
            cam.zoom.radius = desired_zoom;
        } else {
            cam.zoom.radius -= zoom_factor;
        }
    } else {
        if let Some(radius_copy) = cam.zoom.radius_copy {
            let zoom_factor = (radius_copy / cam.aim_zoom) * cam.aim_speed * time.delta_secs();

            // stop zooming out if current radius is greater than original radius
            if cam.zoom.radius >= radius_copy || cam.zoom.radius + zoom_factor >= radius_copy {
                cam.zoom.radius = radius_copy;
                cam.zoom.radius_copy = None;
            } else {
                cam.zoom.radius +=
                    (radius_copy / cam.aim_zoom) * cam.aim_speed * time.delta_secs();
            }
        }
    }
}

pub fn zoom_condition(cam_q: Query<&ThirdPersonCamera, With<ThirdPersonCamera>>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return false;
    };
    return cam.zoom_enabled && cam.cursor_lock_active;
}

// only run toggle_x_offset if `offset_toggle_enabled` is true
fn toggle_x_offset_condition(cam_q: Query<&ThirdPersonCamera, With<ThirdPersonCamera>>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return false;
    };
    cam.offset_toggle_enabled
}

// inverts the x offset. Example: left shoulder view -> right shoulder view & vice versa
fn toggle_x_offset(
    mut cam_q: Query<&mut ThirdPersonCamera, With<ThirdPersonCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    btns: Res<ButtonInput<GamepadButton>>,
) {
    let Ok(mut cam) = cam_q.get_single_mut() else {
        return;
    };

    // check if toggle btn was pressed
    let toggle_btn = keys.just_pressed(cam.offset_toggle_key)
        || btns.just_pressed(cam.gamepad_settings.offset_toggle_button);

    if toggle_btn {
        // Switch direction by inverting the offset_flag
        cam.offset.is_transitioning = !cam.offset.is_transitioning;
    }

    // Determine the transition speed based on direction
    let transition_speed = if cam.offset.is_transitioning {
        -cam.offset_toggle_speed
    } else {
        cam.offset_toggle_speed
    };

    // Update the offset based on the direction and time
    cam.offset.offset.0 = (cam.offset.offset.0 + transition_speed * time.delta_secs())
        .clamp(-cam.offset.offset_copy.0, cam.offset.offset_copy.0);
}

fn toggle_cursor(
    mut cam_q: Query<&mut ThirdPersonCamera>,
    keys: Res<ButtonInput<KeyCode>>,
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut cam) = cam_q.get_single_mut() else {
        return;
    };

    if keys.just_pressed(cam.cursor_lock_key) {
        cam.cursor_lock_active = !cam.cursor_lock_active;
    }

    if let Ok(mut window) = window_q.get_single_mut() {
        if cam.cursor_lock_active {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
        } else {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}

// checks if the toggle cursor functionality is enabled
fn toggle_cursor_condition(cam_q: Query<&ThirdPersonCamera>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return true;
    };
    cam.cursor_lock_toggle_enabled
}
