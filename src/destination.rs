use bevy::app::App;
use bevy::prelude::{Bundle, Component, Plugin, Query, Transform, Update, Vec3};
use bevy_rapier3d::prelude::Velocity;
use crate::target::{Target, RotationEffect, TranslationEffect};


pub struct DestinationPlugin;

impl Plugin for DestinationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_to_destination);
    }
}



#[derive(Component)] pub struct Speed(pub f32);
#[derive(Component)] pub struct RotationSpeed(pub f32);
#[derive(Component)] pub enum Destination {
    Target(Target),
    Reached
}

#[derive(Bundle)]
pub struct DestinationBundle {
    pub destination: Destination,
    pub speed: Speed,
    pub rotation_speed: RotationSpeed
}


impl Destination {
    pub fn new(from: Vec3, target: Vec3) -> Self {

        if (target - from).length() == 0.0 {
            Destination::Reached
        } else {
            Destination::Target(Target::new(from, target))
        }

    }

    pub fn pause(&mut self) {
        if let Destination::Target(target) = self {
            target.pause();
        }
    }

    pub fn resume(&mut self) {
        if let Destination::Target(target) = self {
            target.resume();
        }
    }
}


const DEFAULT_SPEED_VALUE: f32 = 6.0;
const DEFAULT_ROTATION_SPEED_VALUE: f32 = 18.0;

static DEFAULT_SPEED: Speed = Speed(DEFAULT_SPEED_VALUE);
static DEFAULT_ROTATION_SPEED: RotationSpeed = RotationSpeed(DEFAULT_ROTATION_SPEED_VALUE);

impl Default for Speed {
    fn default() -> Self {
        Self(DEFAULT_SPEED_VALUE)
    }
}

impl<'a> Default for &'a Speed {
    fn default() -> Self {
        &DEFAULT_SPEED
    }
}

impl Default for RotationSpeed {
    fn default() -> Self {
        Self(DEFAULT_ROTATION_SPEED_VALUE)
    }
}

impl<'a> Default for &'a RotationSpeed {
    fn default() -> Self {
        &DEFAULT_ROTATION_SPEED
    }
}

fn move_to_destination(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut Destination,
        Option<&Speed>,
        Option<&RotationSpeed>
    )>
) {

    query.for_each_mut( | (mut transform, mut velocity, mut destination, speed, rotation_speed) | {

        velocity.linvel = Vec3::ZERO;
        velocity.angvel = Vec3::ZERO;

        match &mut *destination {
            Destination::Reached => {}

            Destination::Target(target) => {
                let rotation_effect = target.get_rotation_effect(*transform, rotation_speed.unwrap_or_default().0);
                let translation_effect = target.get_translation_effect(*transform, speed.unwrap_or_default().0);

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

                if let (RotationEffect::RotationDone, TranslationEffect::DestinationReached) = (rotation_effect, translation_effect) {
                    *destination = Destination::Reached;
                }
            }
        }
    });
}