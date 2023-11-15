use bevy::prelude::*;

//noinspection DuplicatedCode

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, move_cube_to_the_right_and_rotate_it)
        .run();
}

#[derive(Component)]
struct Cube;

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {


    // Spawn cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::BEIGE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Cube);


    // Spawn Camera
    let player_camera_y_offset: f32 = 20.0;
    let player_camera_z_offset: f32 = 10.0;

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, player_camera_y_offset, player_camera_z_offset)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });



    // Spawn platform for reference
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::SEA_GREEN.into()),
        ..default()
    });


    // Add global light
    commands.insert_resource(AmbientLight {
        color: Default::default(),
        brightness: 1.0,
    });

}

fn move_cube_to_the_right_and_rotate_it(mut query: Query<&mut Transform, With<Cube>>) {
    let mut transform = query.single_mut();

    transform.translation.x += 0.05;
    transform.rotation *= Quat::from_rotation_y(2f32.to_radians());
}
