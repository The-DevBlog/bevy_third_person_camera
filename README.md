# Bevy Third Person Camera

![Demo](assets/demo.gif)

## Getting Started

Install the **bevy_third_person_camera** crate: 

`cargo install bevy_third_person_camera`

Add the third person plugin: 

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

## Default Mouse Controls

- Scroll Wheel - Zoom in/out
- Space - Unlock/lock cursor

## Default Gamepad Controls

- DPad Up - Zoom in
- DPad Down - Zoom out

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
        zoom_bounds: (1.0, 10.0),
        zoom_sensitivity: 1.0,
        gamepad_settings: CustomGamepadSettings {
            x_sensitivity: 7.0,
            y_sensitivity: 4.0,
            zoom_in_button: GamepadButton::new(gamepad, GamepadButtonType::DPadUp),
            zoom_out_button: GamepadButton::new(gamepad, GamepadButtonType::DPadDown),
        },
        ..default()
    },
    Camera3dBundle::default(),
));
```

## Cursor Lock

The cursor lock feature allows the mouse cursor to toggle between a locked, hidden state, to an unlocked, visible state. When unlocked, the orbiting feature is disabled, thus allowing the cursor to move freely within the window without disrupting the camera's transform. This feature can be fully disabled by setting the **enable_cursor_lock_toggle** value to **false** and will keep the cursor locked and hidden.

![Demo](assets/demo2.gif)

## Bevy Version Compatibility

| bevy | bevy_third_person_camera |
| ---- | ------------------------ |
| 0.11 | 0.1.1                    |

## License

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)





