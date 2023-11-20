use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, move_to_destination)
        .run();
}

#[derive(Component)]
struct Cube;

#[derive(Component)]
pub enum Destination {
    Target(Vec3),
    Reached,
}

#[derive(Component)]
pub struct DestinationSpeed {
    pub translation: f32,
    pub rotation: f32
}

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
        .insert(Cube)
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity::default())
        .insert(Destination::Target(Vec3::new(5.0, 0.5, 0.0)))
        .insert(DestinationSpeed {
            translation: 6.0,
            rotation: 18.0,
        });

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

fn move_to_destination(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut Destination,
        Option<&DestinationSpeed>,
    )>,
) {
}
