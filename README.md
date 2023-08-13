# Bevy Third Person Camera

- Zoom in/out
- Orbit
- Custom Offset
- Cursor Lock
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

The `offset` is of type (f32, f32) and will offset x and y values of the camera respectively. By default, the `offset` is set to `(0.0, 0.0)` and the `offset_toggle_key` and `offset_toggle_button` are `None`. 


The following GIF has these values: 
```rust
offset: Offset::new(0.5, 0.25),
offset_toggle_key: Some(KeyCode::T),
offset_toggle_speed: 5.0 // default
```

![offset demo](assets/offsetDemo.gif)

### Cursor Lock

The cursor lock feature allows the mouse cursor to toggle between a locked, hidden state, to an unlocked, visible state. When unlocked, the orbiting feature is disabled, thus allowing the cursor to move freely within the window without disrupting the camera's transform. This feature can be fully disabled by setting the **enable_cursor_lock_toggle** value to **false** and will keep the cursor locked and hidden.

![cursor lock demo](assets/cursorLockDemo.gif)

## Custom Settings

Most settings can be overridden: 

```rust
let gamepad = Gamepad::new(0);
commands.spawn((
    ThirdPersonCamera {
        cursor_lock_key: KeyCode::Space,
        enable_cursor_lock_toggle: true,
        lock_cursor: true,
        mouse_sensitivity: 2.0,
        offset: Offset::new(0.5, 0.25),
        offset_toggle_speed: 5.0,
        offset_toggle_key: Some(KeyCode::T), // default is None
        zoom_bounds: (1.0, 10.0),
        zoom_sensitivity: 1.0,
        gamepad_settings: CustomGamepadSettings {
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

|                    | Mouse/Keyboard | Gamepad    |
| ------------------ | -------------- | ---------- |
| Zoom In            | Scroll Up      | D Pad Up   |
| Zoom Out           | Scroll Down    | D Pad Down |
| Cursor Lock/Unlock | Space          | n/a        |

## Bevy Version Compatibility

| bevy | bevy_third_person_camera |
| ---- | ------------------------ |
| 0.11 | 0.1.2                    |

## License

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)





