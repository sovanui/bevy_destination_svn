use crate::target::{RotationEffect, Target, TranslationEffect};
use bevy::app::App;
use bevy::prelude::{Bundle, Component, Plugin, PostUpdate, Query, Transform, Vec3};
use bevy_rapier3d::prelude::Velocity;

pub struct DestinationPlugin;

impl Plugin for DestinationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, move_to_destination);
    }
}


#[derive(Component, Copy, Clone)]
pub struct DestinationSpeed {
    pub translation: f32,
    pub rotation: f32
}


#[derive(Component)]
pub enum Destination {
    Target(Target),
    Reached,
}

#[derive(Bundle)]
pub struct DestinationBundle {
    pub destination: Destination,
    pub speed: DestinationSpeed
}

impl Destination {
    pub fn new(from: Vec3, target: Vec3) -> Self {
        if (target - from).length() == 0.0 {
            Destination::Reached
        } else {
            Destination::Target(Target::new(from, target))
        }
    }
}

const DEFAULT_SPEED_VALUE: f32 = 6.0;
const DEFAULT_ROTATION_SPEED_VALUE: f32 = 18.0;

static DEFAULT_DESTINATION_SPEED: DestinationSpeed = DestinationSpeed {
    translation: DEFAULT_SPEED_VALUE,
    rotation: DEFAULT_ROTATION_SPEED_VALUE,
};


impl Default for DestinationSpeed {
    fn default() -> Self { DEFAULT_DESTINATION_SPEED }
}

impl<'a> Default for &'a DestinationSpeed {
    fn default() -> Self {
        &DEFAULT_DESTINATION_SPEED
    }
}


#[allow(clippy::type_complexity)]
#[rustfmt::skip]
fn move_to_destination(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut Destination,
        Option<&DestinationSpeed>,
    )>,
) {
    query.for_each_mut(|(mut transform, mut velocity, mut destination, destination_speed)| {
        velocity.linvel = Vec3::ZERO;
        velocity.angvel = Vec3::ZERO;

        match &mut *destination {
            Destination::Reached => {}

            Destination::Target(target) => {
                let rotation_effect = target.get_rotation_effect(*transform, destination_speed.unwrap_or_default().rotation);
                let translation_effect = target.get_translation_effect(*transform, destination_speed.unwrap_or_default().translation);

                match rotation_effect {
                    RotationEffect::AngularVelocity(angular_velocity) => { velocity.angvel = angular_velocity }
                    RotationEffect::FinalRotationFix(final_rotation_fix) => { transform.rotation = final_rotation_fix }
                    RotationEffect::RotationDone => {}
                }

                match translation_effect {
                    TranslationEffect::LinearVelocity(linear_velocity) => { velocity.linvel = linear_velocity }
                    TranslationEffect::FinalTranslationFix(final_translation_fix) => { transform.translation = final_translation_fix }
                    TranslationEffect::DestinationReached => {}
                }

                if let
                    (RotationEffect::RotationDone, TranslationEffect::DestinationReached) =
                    (rotation_effect, translation_effect)
                {
                    *destination = Destination::Reached;
                }
            }
        }
    });
}
