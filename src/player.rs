use bevy::prelude::*;

use crate::Player;

pub fn spawn_player(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_translation(Vec3::new(3.0, 0.5, 0.0)),
            ..default()
        },
        Player,
        Name::new("Player"),
    );

    cmds.spawn(player);
}
