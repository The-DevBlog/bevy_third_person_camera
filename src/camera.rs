use bevy::prelude::*;

use crate::{CustomCamera, Player};

// pub fn spawn_camera(mut cmds: Commands, player_q: Query<&Transform, With<Player>>) {
pub fn spawn_camera(mut cmds: Commands, player_q: Query<&Transform, With<Player>>) {
    let trans = Vec3::new(0.0, 10.0, 10.0);

    // let player_trans = player_q.get_single();

    // match player_trans {
    //     Ok(e) => println!("GOOD: {:?}", e),
    //     Err(e) => println!("BAD: {}", e),
    // };

    // let player_trans = player_q;

    if let Ok(player_trans) = player_q.get_single() {
        println!("COOOOOOOOOOOOOOOOOOOL {}", player_trans.translation);
    }

    let cam = (
        Camera3dBundle {
            // transform: Transform::from_translation(trans).looking_at(Vec3::ZERO, Vec3::Y),
            // transform: Transform::from_translation(trans)
            //     .looking_at(player_trans.translation, Vec3::Y),
            ..default()
        },
        CustomCamera,
        Name::new("Camera"),
    );

    cmds.spawn(cam);
}

pub fn update_camera_position(
    mut cam_q: Query<&mut Transform, With<CustomCamera>>,
    player_q: Query<&Transform, With<Player>>,
) {
    // if let Ok(cam_trans) = cam_q.get_single_mut() {
    //     if let Ok(player_q) = player_q.get_single_mut() {
    //         // cam_trans.look_at(target, up)
    //     }
    // }
}
