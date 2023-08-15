# Bevy Third Person Camera

- Aim
- Zoom in/out
- Orbit
- Custom Offset
- Cursor Lock
- Custom Sensitivity
- Full Gamepad Support!

![camera demo](assets/cameraDemo.gif)

## Getting Started

Add the **bevy_third_person_camera** crate: 

```
cargo add bevy_third_person_camera
```

Add the **ThirdPersonPlugin**: 

```rust
.add_plugins(ThirdPersonCameraPlugin)
```

Add the **ThirdPersonCamera** component to the camera entity: 

```rust
commands.spawn((
    ThirdPersonCamera::default(),
    Camera3dBundle::default()
));
```

Add the **ThirdPersonCameraTarget** component to your player:

```rust
// Player
commands.spawn((
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    },
    ThirdPersonCameraTarget,
    Player,
));
```

That's it! 

## Examples

- default
- custom

```
cargo run --example <example name>
```

## Features

### Offset

The `offset` will 'offset' the x and y values of the camera respectively. Offset is disabled by default. Turn on with `offset_enabled: true` 

```rust
offset_enabled: true,
offset: Offset::new(0.5, 0.25),
offset_toggle_key: Some(KeyCode::T),
offset_toggle_speed: 5.0 // default
```

![offset demo](assets/offsetDemo.gif)

### Aim

Aiming is calculated using the `aim_zoom` & the `zoom.min` values. Please note that the actual zoom level will vary if you  change the `zoom.min` value, even if the `aim_zoom` value stays the same. Aiming is disabled by default. Turn on with `aim_enabled: true`

```rust
aim_enabled: true, // default
aim_speed: 3.0, // default
aim_zoom: 0.7, // default
aim_button: Some(MouseButton::Right), // default
zoom: Zoom::new(1.5, 3.0)
```
![aim demo](assets/aimDemo.gif)

### Cursor Lock

The cursor lock feature allows the mouse cursor to toggle between a locked, hidden state, to an unlocked, visible state. When unlocked, the orbiting feature is disabled, thus allowing the cursor to move freely within the window without disrupting the camera's transform. This feature can be fully disabled by setting the **enable_cursor_lock_toggle** value to **false** and will keep the cursor locked and hidden.

![cursor lock demo](assets/cursorLockDemo.gif)

## Custom Settings

Most settings can be overridden: 

```rust
let gamepad = Gamepad::new(0);
commands.spawn((
    // These are the default settings
    ThirdPersonCamera {
        aim_enabled: false,
        aim_button: Some(MouseButton::Right),
        aim_speed: 3.0,
        aim_zoom: 0.7,
        cursor_lock_key: KeyCode::Space,
        enable_cursor_lock_toggle: true,
        lock_cursor: true,
        mouse_sensitivity: 2.0,
        offset_enabled: false,
        offset: Offset::new(0.5, 0.4),
        offset_toggle_speed: 5.0,
        offset_toggle_key: Some(KeyCode::T), // default is None
        zoom: Zoom::new(1.5, 3.0),
        zoom_sensitivity: 1.0,
        gamepad_settings: CustomGamepadSettings {
            aim_button: Some(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2)),
            x_sensitivity: 7.0,
            y_sensitivity: 4.0,
            offset_toggle_button: Some(GamepadButton::new(gamepad, GamepadButtonType::DPadRight)), // default is None
            zoom_in_button: GamepadButton::new(gamepad, GamepadButtonType::DPadUp),
            zoom_out_button: GamepadButton::new(gamepad, GamepadButtonType::DPadDown),
        },
        ..default()
    },
    Camera3dBundle::default(),
));
```

## Default Controls

|                    | Mouse/Keyboard     | Gamepad      |
| ------------------ | ------------------ | ------------ |
| Zoom In            | Scroll Up          | D Pad Up     |
| Zoom Out           | Scroll Down        | D Pad Down   |
| Aim                | Right Mouse Button | Left Trigger |
| Cursor Lock/Unlock | Space              | n/a          |

## Bevy Version Compatibility

| bevy | bevy_third_person_camera |
| ---- | ------------------------ |
| 0.11 | 0.1.3                    |

## License

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)





