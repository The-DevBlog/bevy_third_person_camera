use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod camera;
pub mod player;
pub mod world;

use camera::*;
use player::*;
use world::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_systems((setup_world, spawn_camera, spawn_player))
        .run();
}

// Components
#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct CustomCamera;
