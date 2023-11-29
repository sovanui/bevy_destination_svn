use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_destination_svn::destination::{Destination, DestinationBundle, DestinationPlugin, DestinationSpeed};

fn main() {
    App::new().
        add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            DestinationPlugin
    ))
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, set_next_destination)
        .add_systems(Update, update_speed)
        .add_systems(Update, update_rotation_speed)
        .run();
}


#[derive(Component)]
struct DestinationMarker;


fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let starting_point = Vec3::new(0.0, 0.5, 0.0);
    let first_destination = Vec3::new(5.0, 0.5, 5.0);

    // Spawn object
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: materials.add(Color::BEIGE.into()),
        transform: Transform::from_translation(starting_point),
        ..default()
    })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity::default())
        .insert(DestinationBundle {
            destination: Destination::new(starting_point, first_destination),
            speed: DestinationSpeed::default(),
        });

    // Spawn destination marker
    commands.spawn( PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.40,
            ..default()
        })),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_translation(first_destination),
        ..default()
    }).insert(DestinationMarker);


    let player_camera_y_offset: f32 = 20.0;
    let player_camera_z_offset: f32 = 10.0;

    // Spawn Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, player_camera_y_offset, player_camera_z_offset)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn platform
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(30.0))),
        material: materials.add(Color::SEA_GREEN.into()),
        ..default()
    });


    // Add global light
    commands.insert_resource(AmbientLight {
        color: Default::default(),
        brightness: 1.0,
    });

}

fn set_next_destination(
    mut query: Query<(&mut Destination, &Transform)>,
    mut destination_marker: Query<&mut Transform, (With<DestinationMarker>, Without<Destination>)>
) {


    query.for_each_mut(|(mut destination, transform)| {
        match &mut *destination {

            Destination::Target(_) => {}

            // set new destination when reached previous
            Destination::Reached => {

                let next_destination = Vec3::new(
                    rand::random::<f32>() * 20.0 - 10.0,
                    0.5,
                    rand::random::<f32>() * 20.0 - 10.0
                );

                *destination = Destination::new(transform.translation, next_destination);

                let mut marker_transform = destination_marker.single_mut();
                marker_transform.translation = next_destination;
            }
        }
    });

}


fn update_speed(
    keyboard_inputs: Res<Input<KeyCode>>,
    mut query: Query<&mut DestinationSpeed>
) {
    query.for_each_mut(|mut speed| {
        if keyboard_inputs.just_pressed(KeyCode::Up) {
            speed.translation += 1.0;
        }

        if keyboard_inputs.just_pressed(KeyCode::Down) {
            speed.translation -= 1.0;
        }
    });
}

fn update_rotation_speed(
    keyboard_inputs: Res<Input<KeyCode>>,
    mut query: Query<&mut DestinationSpeed>
) {
    query.for_each_mut(|mut rotation_speed| {
        if keyboard_inputs.just_pressed(KeyCode::Right) {
            rotation_speed.rotation += 2.0;
        }

        if keyboard_inputs.just_pressed(KeyCode::Left) {
            rotation_speed.rotation -= 2.0;
        }
    });
}
