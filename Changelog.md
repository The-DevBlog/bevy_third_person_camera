# v0.1.4

#### <ins>!Breaking Changes!</ins>

- Renamed `enable_cursor_lock_toggle` to `cursor_lock_toggle_enabled` for standardization
- Renamed `lock_cursor` to `cursor_lock_active` for standardization
- Changed type of `aim_button` from `Some(MouseButton)` to `MouseButton` for simplicity
- Changed type of `offset_toggle_key` from `Some(KeyCode)` to `KeyCode` for simplicity
- Changed type of `CustomGamepadSettings.aim_button` from `Some(GamepadButton)` to `GamepadButton` for simplicity
- Changed type of `CustomGamepadSettings.zoom_in_button` from `Some(GamepadButton)` to `GamepadButton` for simplicity
- Changed type of `CustomGamepadSettings.zoom_out_button` from `Some(GamepadButton)` to `GamepadButton` for simplicity

#### Minor Changes

- Added `cursor_lock_toggle_enabled`, & `offset_toggle_enabled` to easily be able to turn on/off features

# v0.1.3

#### <ins>!Breaking Changes!</ins>

- **Zoom** has been switched from type `(f32, f32)` to a custom type `Zoom`.

#### Minor Changes

- Added `offset_enabled` to easily control if offset should be applied or not
- Offset is now disabled by default
- Conditions have been added to `toggle_x_offset`. Now that system will *only* run if `offset_enabled` is true
- Added Changelog
  
#### Features

- Aim
  - off by default
  - rotates the target to face direction he is aiming
  - custom aim zoom and aim speed
![aim demo](assets/aimDemo.gif)

# v0.1.2

#### Features

- Offset
![offset demo](assets/offsetDemo.gif)

# v0.1.1

Initial Release!