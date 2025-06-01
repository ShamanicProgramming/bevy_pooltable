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
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.54, 1.27))),
        MeshMaterial3d(green_table_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.54, 0.08, 0.08))),
        MeshMaterial3d(brown_table_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.635),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.54, 0.08, 0.08))),
        MeshMaterial3d(brown_table_material.clone()),
        Transform::from_xyz(0.0, 0.0, -0.635),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.08, 0.08, 1.27))),
        MeshMaterial3d(brown_table_material.clone()),
        Transform::from_xyz(1.27, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.08, 0.08, 1.27))),
        MeshMaterial3d(brown_table_material.clone()),
        Transform::from_xyz(-1.27, 0.0, 0.0),
    ));
}

pub fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn add_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 4.0, 2.0),
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
        (0.614, 0.0254, 0.0508),
        (0.614, 0.0254, -0.0508),
        (0.5632, 0.0254, -0.0254),
        (0.5632, 0.0254, 0.0254),
        (0.5124, 0.0254, 0.0),
        (0.6648, 0.0254, 0.0254),
        (0.6648, 0.0254, -0.0254),
        (0.6648, 0.0254, -0.0762),
        (0.6648, 0.0254, 0.0762),
        (0.7156, 0.0254, 0.0),
        (0.7156, 0.0254, -0.0508),
        (0.7156, 0.0254, 0.0508),
        (0.7156, 0.0254, 0.1016),
        (0.7156, 0.0254, -0.1016),
    ];

    let mut rng = rng();
    ball_textures.shuffle(&mut rng);

    // eight ball
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.0254))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(eight_ball_texture.clone()),
            ..default()
        })),
        Transform::from_xyz(0.614, 0.0254, 0.0),
    ));

    // cue ball
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.0254))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(cue_ball_texture.clone()),
            ..default()
        })),
        Transform::from_xyz(-0.614, 0.0254, 0.0),
    ));

    // other balls
    for i in 0..14 {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.0254))),
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
