use std::f32::consts::TAU;

use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_inspector_egui::egui::lerp;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct Speed(f32);

#[derive(Component, Debug)]
struct Locked(bool);

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed, &mut Locked), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    for (mut player_transform, player_speed, mut player_locked) in player_query.iter_mut() {
        let cam = match camera_query.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retriving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;
        let rotation_margin = 0.25f32;
        let rotation_snap = 0.01f32;

        // forward
        if keys.pressed(KeyCode::KeyW) {
            direction += cam.forward().xyz();
        }

        // back
        if keys.pressed(KeyCode::KeyS) {
            direction += cam.back().xyz();
        }

        // left
        if keys.pressed(KeyCode::KeyA) {
            direction += cam.left().xyz();
        }

        // right
        if keys.pressed(KeyCode::KeyD) {
            direction += cam.right().xyz();
        }

        if keys.just_pressed(KeyCode::Space) {
            if player_locked.0 == false {
                player_locked.0 = true;
            } else {
                player_locked.0 = false;
            }
        }

        direction.y = 0.0;

        for ev in motion_evr.read() {
            if player_locked.0 == false {
                player_transform.rotate_x(
                    ev.delta.y.to_radians() * player_speed.0 * time.delta_seconds() * 1000.0,
                );
                player_transform.rotate_y(
                    ev.delta.x.to_radians() * player_speed.0 * time.delta_seconds() * 1000.0,
                );
            }
        }

        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        if movement.length() > 0.0 {
            player_transform.translation += movement;
            println!(
                "Movement is: {}. player position is {}",
                movement, player_transform.translation
            );
        }

        let current_rotation = player_transform.rotation.normalize();
        let target_rotation =
            Quat::from_vec4(Vec4::new(0.7196, -0.0829, 0.01618, 0.7138461)).normalize();

        if target_rotation.angle_between(current_rotation) <= rotation_margin {
            player_locked.0 = true;
            if target_rotation.angle_between(current_rotation) <= f32::EPSILON {
                println!("Here");
                player_transform.rotation = target_rotation;
            } else {
                player_transform.rotation = player_transform.rotation.slerp(
                    target_rotation,
                    player_speed.0 * 10.0 * time.delta_seconds(),
                );
            }
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
	meshes: Res<Assets<Mesh>>
) {
    // let flashlight = (
    //     SpotLightBundle {
    //         spot_light: SpotLight {
    //             color: Color::rgba_u8(236, 234, 91, 255),
    //             intensity: 4000.0,
    //             outer_angle: 0.6,
    //             inner_angle: 0.5,
    //             shadows_enabled: true,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(0.0, 0.25, -0.2),
    //         ..default()
    //     },
    //     Name::new("Flashlight"),
    // );

    let mesh_handler = assets.load("logo-4.obj");

	if let Some(mesh) = meshes.get(&mesh_handler) {
		
	}



    let player = (
        // SceneBundle {
        //     scene: assets.load("globe-earth.obj"),
        //     transform: Transform::from_xyz(-2.6, 8.5, -4.7).with_scale(Vec3::new(0.05, 0.05, 0.05)),
        //     ..default()
        // },
        PbrBundle {
            mesh: mesh_handler,
            material: materials.add(<bevy::prelude::Color as Into<StandardMaterial>>::into(
                Color::DARK_GREEN,
            )),
            transform: Transform::from_xyz(-2.6, 8.5, -4.7).with_scale(Vec3::new(0.5, 0.5, 0.5)),
            ..default()
        },
        Player,
        Speed(0.2),
        Locked(false),
        // ThirdPersonCameraTarget,
        Name::new("Player"),
    );

    commands.spawn(player);
    // commands.spawn(player).with_children(|parent| {
    //     parent.spawn(flashlight);
    // });
}
