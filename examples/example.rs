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
        .add_systems(Update, update_time_speed)
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
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::BEIGE),
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
        mesh: meshes.add(Sphere::new(0.4).mesh().ico(5).unwrap()),
        material: materials.add(Color::RED),
        transform: Transform::from_translation(first_destination),
        ..default()
    }).insert(DestinationMarker);


    let player_camera_y_offset: f32 = 25.0;
    let player_camera_z_offset: f32 = 10.0;

    // Spawn Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, player_camera_y_offset, player_camera_z_offset)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn platform
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(30.0, 30.0)),
        material: materials.add(Color::SEA_GREEN),
        ..default()
    });


    // Add global light
    commands.insert_resource(AmbientLight {
        color: Default::default(),
        brightness: 1000.0,
    });

}

fn set_next_destination(
    mut query: Query<(&mut Destination, &Transform)>,
    mut destination_marker: Query<&mut Transform, (With<DestinationMarker>, Without<Destination>)>
) {


    query.iter_mut().for_each(|(mut destination, transform)| {
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
    keyboard_inputs: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut DestinationSpeed>
) {
    query.iter_mut().for_each(|mut speed| {
        if keyboard_inputs.just_pressed(KeyCode::ArrowUp) { speed.translation += 1.0; }
        if keyboard_inputs.just_pressed(KeyCode::ArrowDown) { speed.translation -= 1.0; }
    });
}

fn update_rotation_speed(
    keyboard_inputs: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut DestinationSpeed>
) {
    query.iter_mut().for_each(|mut rotation_speed| {
        if keyboard_inputs.just_pressed(KeyCode::ArrowRight) { rotation_speed.rotation += 2.0; }
        if keyboard_inputs.just_pressed(KeyCode::ArrowLeft) { rotation_speed.rotation -= 2.0; }
    });
}

fn update_time_speed(
    keyboard_inputs: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>
) {
    if keyboard_inputs.just_pressed(KeyCode::KeyG) {
        let new_speed = time.relative_speed() - 0.2;
        time.set_relative_speed(new_speed);
    }
    if keyboard_inputs.just_pressed(KeyCode::KeyH) {
        let new_speed = time.relative_speed() + 0.2;
        time.set_relative_speed(new_speed);
    }
}
