use bevy::prelude::*;


//noinspection DuplicatedCode

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_scene)
        .run();
}


fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // Spawn platform for reference
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(40.0))),
        material: materials.add(Color::GRAY.into()),
        ..default()
    });


    // Add global light
    commands.spawn(DirectionalLightBundle::default());

    let player_camera_y_offset: f32 = 20.0;
    let player_camera_z_offset: f32 = 10.0;


    // Spawn Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, player_camera_y_offset, player_camera_z_offset)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });


    // Spawn object
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: materials.add(Color::BEIGE.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}