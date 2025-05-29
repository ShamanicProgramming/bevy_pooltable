use bevy::prelude::*;

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
        Transform::from_xyz(2.0, 3.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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
