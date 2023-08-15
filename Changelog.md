# v0.1.3

#### <ins>!Breaking Changes!</ins>
- **Zoom** has been switched from type `(f32, f32)` to a custom type `Zoom`.

#### Other Changes

- Added `offset_enabled` to easily control if offset should be applied or not.
- Offset is now disabled by default. 
- Conditions have been added to `toggle_x_offset`. Now that system will *only* run if `offset_enabled` is true.
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