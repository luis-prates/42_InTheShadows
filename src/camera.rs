use bevy::{prelude::*, render::camera::Exposure};
use bevy_third_person_camera::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            // -4.7, 14.0, 0.0
            transform: Transform::from_xyz(-7.3, 11.1, -1.2)
                .looking_to(Vec3::ZERO, Vec3::Y)
                .with_rotation(Quat::from_euler(EulerRot::XYZ, -0.4, -0.6, -0.2)),
            ..default()
        },
        Name::new("Camera"),
        // ThirdPersonCamera {
        // 	..default()
        // },
    );

    commands.spawn(camera);
}
