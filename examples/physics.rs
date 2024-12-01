/*

!!!OUTDATED!!! - waiting on bevy_rapier3d to support Bevy 0.15

Example displaying the integration with a third party physics engine. In this case Bevy Rapier 3d.
The key is to run the CameraSyncSet AFTER the PhysicsSet, see line 19.
*/

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ThirdPersonCameraPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .add_systems(Startup, (spawn_player, spawn_world, spawn_camera))
        .add_systems(Update, player_movement_keyboard)
        .configure_sets(PostUpdate, CameraSyncSet.after(PhysicsSet::StepSimulation)) // DO THIS!
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let player = (
        SceneRoot(assets.load("Player.gltf#Scene0")),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Collider::cuboid(0.25, 0.5, 0.25),
        RigidBody::Dynamic,
        Damping {
            linear_damping: 5.0,
            ..default()
        },
        ExternalImpulse::default(),
        Player,
        ThirdPersonCameraTarget,
        Speed(4.0),
    );
    commands.spawn(player);
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ThirdPersonCamera {
            aim_enabled: true,
            aim_speed: 3.0, // default
            aim_zoom: 0.7,  // default
            offset_enabled: true,
            offset_toggle_enabled: true,
            gamepad_settings: CustomGamepadSettings { ..default() },
            zoom_enabled: true,        // default
            zoom: Zoom::new(1.5, 3.0), // default
            ..default()
        },
    );
    commands.spawn(camera);
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,

    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        Mesh3d(meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0)))),
        MeshMaterial3d(materials.add(Color::srgb(0.11, 0.27, 0.16))),
        Collider::cuboid(15.0 / 2.0, 0.0 / 2.0, 15.0 / 2.0),
    );

    let light = (
        PointLight {
            intensity: 1500.0 * 1000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    );

    commands.spawn(floor);
    commands.spawn(light);
}

fn player_movement_keyboard(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut ExternalImpulse, &mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut ext_impulse, mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::KeyW) {
            direction += *cam.forward();
        }

        // back
        if keys.pressed(KeyCode::KeyS) {
            direction += *cam.back();
        }

        // left
        if keys.pressed(KeyCode::KeyA) {
            direction += *cam.left();
        }

        // right
        if keys.pressed(KeyCode::KeyD) {
            direction += *cam.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_secs();
        ext_impulse.impulse += movement;

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}
