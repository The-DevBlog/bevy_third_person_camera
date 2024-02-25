mod keyboard;

use bevy::prelude::*;
use keyboard::KeyboardPlugin;

pub struct ThirdPersonControllerPlugin;

impl Plugin for ThirdPersonControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(KeyboardPlugin);
    }
}

#[derive(Component)]
pub struct ThirdPersonController;
