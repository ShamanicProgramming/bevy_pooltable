use std::f32::consts::PI;

use crate::{components::*, constants::CAMERA_HEIGHT};
use bevy::prelude::*;
use rand::{rng, seq::SliceRandom};

pub fn add_table(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let green_table_material = materials.add(Color::srgb_u8(10, 108, 3));
    let brown_table_material = materials.add(Color::srgb_u8(102, 51, 0));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(25.4, 12.7))),
        MeshMaterial3d(green_table_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(25.4, 0.8, 0.8))),
        MeshMaterial3d(brown_table_material.clone()),
        Transform::from_xyz(0.0, 0.0, 6.35),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(25.4, 0.8, 0.8))),
        MeshMaterial3d(brown_table_material.clone()),
        Transform::from_xyz(0.0, 0.0, -6.35),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.8, 0.8, 12.7))),
        MeshMaterial3d(brown_table_material.clone()),
        Transform::from_xyz(12.7, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.8, 0.8, 12.7))),
        MeshMaterial3d(brown_table_material.clone()),
        Transform::from_xyz(-12.7, 0.0, 0.0),
    ));
}

pub fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-17.0, CAMERA_HEIGHT, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn add_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
    ));
}

pub fn add_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cue_ball_texture: Handle<Image> = asset_server.load("textures/BallCue.png");
    let eight_ball_texture: Handle<Image> = asset_server.load("textures/Ball8.png");

    let mut ball_textures: [Handle<Image>; 14] = [
        asset_server.load("textures/Ball1.png"),
        asset_server.load("textures/Ball2.png"),
        asset_server.load("textures/Ball3.png"),
        asset_server.load("textures/Ball4.png"),
        asset_server.load("textures/Ball5.png"),
        asset_server.load("textures/Ball6.png"),
        asset_server.load("textures/Ball7.png"),
        asset_server.load("textures/Ball9.png"),
        asset_server.load("textures/Ball10.png"),
        asset_server.load("textures/Ball11.png"),
        asset_server.load("textures/Ball12.png"),
        asset_server.load("textures/Ball13.png"),
        asset_server.load("textures/Ball14.png"),
        asset_server.load("textures/Ball15.png"),
    ];

    let ball_positions = [
        (6.14, 0.254, 0.508),
        (6.14, 0.254, -0.508),
        (5.632, 0.254, -0.254),
        (5.632, 0.254, 0.254),
        (5.124, 0.254, 0.0),
        (6.648, 0.254, 0.254),
        (6.648, 0.254, -0.254),
        (6.648, 0.254, -0.762),
        (6.648, 0.254, 0.762),
        (7.156, 0.254, 0.0),
        (7.156, 0.254, -0.508),
        (7.156, 0.254, 0.508),
        (7.156, 0.254, 1.016),
        (7.156, 0.254, -1.016),
    ];

    let mut rng = rng();
    ball_textures.shuffle(&mut rng);

    // eight ball
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.254))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(eight_ball_texture.clone()),
            ..default()
        })),
        Transform::from_xyz(6.14, 0.254, 0.0),
    ));

    // cue ball
    commands.spawn((
        Velocity {
            direction: Vec3::default(),
            speed: 0.0,
        },
        CueBall,
        Mesh3d(meshes.add(Sphere::new(0.254))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(cue_ball_texture.clone()),
            ..default()
        })),
        Transform::from_xyz(-6.14, 0.254, 0.0),
    ));

    // other balls
    for i in 0..14 {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.254))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(ball_textures[i].clone()),
                ..default()
            })),
            Transform::from_xyz(
                ball_positions[i].0,
                ball_positions[i].1,
                ball_positions[i].2,
            ),
        ));
    }
}
