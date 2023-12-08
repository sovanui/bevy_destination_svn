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

#[derive(Copy, Clone)]
pub struct Target {
    target: Vec3,
    direction: Vec3,
    last_remaining_distance: f32,
}

impl Target {
    pub fn new(origin: Vec3, target: Vec3) -> Self {
        if (target - origin).length() == 0.0 {
            panic!("Target of length 0");
        }

        Self {
            target,
            direction: (target - origin).normalize(),
            last_remaining_distance: origin.distance(target)
        }
    }
}


#[derive(Component)]
pub enum Destination {
    Target(Target),
    Reached,
}

impl Destination {
    pub fn new(origin: Vec3, target: Vec3) -> Self {
        if (target - origin).length() == 0.0 {
            Destination::Reached
        } else {
            Destination::Target(Target::new(origin, target))
        }
    }
}

#[derive(Component)]
pub struct DestinationSpeed {
    pub translation: f32,
    pub rotation: f32,
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn cube

    let cube_start_point = Vec3::new(0.0, 0.5, 0.0);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::BEIGE.into()),
            transform: Transform::from_translation(cube_start_point),
            ..default()
        })
        .insert(Cube)
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity::default())
        .insert(Destination::new(cube_start_point, Vec3::new(5.0, 0.5, 0.0)))
        .insert(DestinationSpeed {
            translation: 6.0,
            rotation: 3.5,
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

#[rustfmt::skip]
fn move_to_destination(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut Destination,
        &DestinationSpeed
    )>,
) {
    query.for_each_mut(|(mut transform, mut velocity, mut destination, destination_speed)| {

        if let Destination::Target(target) = &mut *destination {

            // Translation
            const REACH_TRANSLATION_RATIO: f32 = 1.0 / 120.0;
            let reached_translation_threshold: f32 = destination_speed.translation * REACH_TRANSLATION_RATIO;

            let remaining_distance = transform.translation.distance(target.target);
            let last_remaining_distance = target.last_remaining_distance;
            target.last_remaining_distance = last_remaining_distance;

            let has_reached_translation = remaining_distance <= reached_translation_threshold;
            let has_gone_past_translation = remaining_distance > last_remaining_distance;
            let translation_reached =  has_reached_translation || has_gone_past_translation;


            if translation_reached {

                velocity.linvel = Vec3::ZERO;
                transform.translation = target.target;

            } else {

                velocity.linvel = target.direction.normalize() * destination_speed.translation;

            }

            // Rotation
            const REACH_ROTATION_RATIO: f32 = 1.0 / 60.0;
            let reached_rotation_threshold: f32 = destination_speed.rotation * REACH_ROTATION_RATIO;

            let current_direction = transform.forward();
            let angle_between = current_direction.angle_between(target.direction);
            let rotation_reached = angle_between <= reached_rotation_threshold;

            if rotation_reached {
                velocity.angvel = Vec3::ZERO;
                transform.rotation = transform.looking_to(target.direction, Vec3::Y).rotation;
            } else {
                let rotation_axis = current_direction.cross(target.direction).normalize();
                velocity.angvel = rotation_axis * destination_speed.rotation;
            }


            // Check if destination reached
            if translation_reached && rotation_reached {
                *destination = Destination::Reached;
            }
        }
    });
}
