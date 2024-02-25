pub mod camera;
pub mod controller;

use bevy::prelude::*;
use camera::ThirdPersonCameraPlugin;

pub struct CustomPlugin;

impl Plugin for CustomPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ThirdPersonCameraPlugin);
    }
}
