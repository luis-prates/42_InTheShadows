use std::f32::consts::PI;

use bevy::{prelude::*, render::view::RenderLayers};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_light, spawn_floor, spawn_wall, spawn_objects),
        );
    }
}

fn spawn_light(mut commands: Commands) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                intensity: 1000000.0,
                shadows_enabled: false,
                color: Color::rgba(255.0 / 255.0, 199.0 / 255.0, 0.0, 255.0 / 255.0),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Name::new("Main light"),
    );

    let front_light = (
        PointLightBundle {
            point_light: PointLight {
                intensity: 50000000.0,
                shadows_enabled: true,
                color: Color::rgba_u8(255, 199, 0, 255),
                ..default()
            },
            transform: Transform::from_xyz(-2.6, 8.5, 2.0),
            ..default()
        },
        Name::new("Front Light"),
    );

    commands.spawn(light);
    commands.spawn(front_light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0))),
            material: materials.add(<bevy::prelude::Color as Into<StandardMaterial>>::into(
                Color::DARK_GREEN,
            )),
            ..default()
        },
        Name::new("Floor"),
    );

    commands.spawn(floor);
}

fn spawn_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0))),
            material: materials.add(<bevy::prelude::Color as Into<StandardMaterial>>::into(
                Color::DARK_GREEN,
            )),
            transform: Transform::from_xyz(0.0, 7.5, -7.5)
                .with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0)),
            ..default()
        },
        Name::new("Wall"),
    );

    commands.spawn(wall);
}

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut create_cubes =
        |size: f32, color: Color, xyz: (f32, f32, f32), name: String| -> (PbrBundle, Name) {
            (
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(size, size, size))),
                    material: materials.add(
                        <bevy::prelude::Color as Into<StandardMaterial>>::into(color),
                    ),
                    transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
                    ..default()
                },
                Name::new(name),
            )
        };

    commands.spawn(create_cubes(
        4.0,
        Color::BLUE,
        (-5.0, 2.0, 5.0),
        "Blue Cube".to_string(),
    ));

    commands.spawn(create_cubes(
        2.0,
        Color::RED,
        (6.0, 1.0, -6.0),
        "Red Cube".to_string(),
    ));
}
