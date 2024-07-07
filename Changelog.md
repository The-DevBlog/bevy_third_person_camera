# v0.1.11 (not published)

## <ins>!Breaking Changes!<ins>

View the [migration guide here](migrationGuides/v0.1.10-v0.1.11.md)

- Completely removed the controller logic
  - I regretted adding this initially. This has no place in the camera crate itself, as it serves a different purpose. I would like to keep this crate focused on the camera logic only going forward.
- Changed ThirdPersonCamera field 'mouse_sensitivity' type to Vec2
- Replaced CameraGamepadSettings fields 'x_sensitivity' & 'y_sensitivity' fields with 'sensitivity' field with type Vec2

## Minor Changes

- Upgrade to Bevy v0.14.0
- Added more documentation

# v0.1.10

## <ins>!Breaking Changes!</ins>

View the [migration guide here](migrationGuides/v0.1.9-v0.1.10.md)

- Removed 'focus' field as it had zero effect
- Renamed 'CustomGamepadSettings' to 'CameraGamepadSettings' for standardization
- Moved 'CameraGamepadSettings', 'Offset' & 'Zoom' to 'Camera' module

## New Features

- Third Person Controller (deprecated)
  - A new component that will add basic controls to the ThirdPersonCameraTarget
  - WASD movement controls 
  - Sprint Key/Button
  - Customizable layout
  - As always, full Gamepad support!

# v0.1.9

- Updated Bevy to latest version 0.13.0 [KyleWinston](https://github.com/KyleWinston)

# v0.1.8

- Updated Bevy to latest version 0.12.1
- Updated Bevy dependency to disable bevy default features and add full feature set [shenjiangqiu](https://github.com/shenjiangqiu)

# v0.1.7

- Updated Bevy to latest version 0.12.0

# v0.1.6

- Updated Bevy to latest version 0.11.3

## Bug Fixes

- Added an additional zoom condition so that zooming does not occur when the cursor lock is enabled - [Matthewwyndham](https://github.com/matthewwyndham)

# v0.1.5

## New Features

- Conditional Orbit - [SGoerzen](https://github.com/SGoerzen)
  - Orbiting can now be activated using a custom mouse button
  - Default is off, so orbiting will work like normal

# v0.1.4

## <ins>!Breaking Changes!</ins>

- Renamed `enable_cursor_lock_toggle` to `cursor_lock_toggle_enabled` for standardization
- Renamed `lock_cursor` to `cursor_lock_active` for standardization
- Changed type of `aim_button` from `Some(MouseButton)` to `MouseButton` for simplicity
- Changed type of `offset_toggle_key` from `Some(KeyCode)` to `KeyCode` for simplicity
- Changed type of `CustomGamepadSettings.aim_button` from `Some(GamepadButton)` to `GamepadButton` for simplicity
- Changed type of `CustomGamepadSettings.zoom_in_button` from `Some(GamepadButton)` to `GamepadButton` for simplicity
- Changed type of `CustomGamepadSettings.zoom_out_button` from `Some(GamepadButton)` to `GamepadButton` for simplicity

## Minor Changes

- Added `cursor_lock_toggle_enabled`, & `offset_toggle_enabled` to easily be able to turn on/off features

# v0.1.3

## <ins>!Breaking Changes!</ins>

- **Zoom** has been switched from type `(f32, f32)` to a custom type `Zoom`.

## Minor Changes

- Added `offset_enabled` to easily control if offset should be applied or not
- Offset is now disabled by default
- Conditions have been added to `toggle_x_offset`. Now that system will *only* run if `offset_enabled` is true
- Added Changelog
  
## New Features

- Aim
  - off by default
  - rotates the target to face direction he is aiming
  - custom aim zoom and aim speed
![aim demo](assets/aimDemo.gif)

# v0.1.2

## New Features

- Offset
![offset demo](assets/offsetDemo.gif)

# v0.1.1

Initial Release!