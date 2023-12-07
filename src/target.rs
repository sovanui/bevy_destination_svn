use bevy::prelude::{Quat, Transform, Vec3};

struct TargetState {
    last_remaining_distance: f32,
    rotation_done: bool,
    translation_done: bool,
}


pub struct Target {
    target: Vec3,
    direction: Vec3,
    state: TargetState,
}

const LINEAR_THRESHOLD_RATIO: f32 = 1.0 / 120.0;
const ANGULAR_THRESHOLD_RATIO: f32 = 1.0 / 60.0;

pub enum RotationEffect {
    AngularVelocity(Vec3),
    FinalRotationFix(Quat),
    RotationDone,
}

pub enum TranslationEffect {
    LinearVelocity(Vec3),
    FinalTranslationFix(Vec3),
    DestinationReached,
}

impl Target {
    pub fn new(origin: Vec3, target: Vec3) -> Self {
        if (target - origin).length() == 0.0 {
            panic!("Target of length 0");
        }

        Self {
            target,
            direction: (target - origin).normalize(),
            state: TargetState {
                last_remaining_distance: origin.distance(target),
                rotation_done: false,
                translation_done: false,
            },
        }
    }

    pub fn set_translation_done(&mut self) {
        self.state.translation_done = true;
    }

    pub fn get_rotation_effect(
        &mut self,
        transform: Transform,
        rotation_speed: f32,
    ) -> RotationEffect {
        if self.state.rotation_done {
            return RotationEffect::RotationDone;
        }

        if self.is_in_facing_threshold(transform, rotation_speed) {
            self.state.rotation_done = true;
            return RotationEffect::FinalRotationFix(
                transform.looking_to(self.direction, Vec3::Y).rotation,
            );
        }

        RotationEffect::AngularVelocity(
            self.get_needed_angular_velocity(transform) * rotation_speed,
        )
    }

    pub fn get_translation_effect(
        &mut self,
        transform: Transform,
        speed: f32,
    ) -> TranslationEffect {
        if self.state.translation_done {
            return TranslationEffect::DestinationReached;
        }

        if self.has_reached_destination(transform, speed) {
            self.state.translation_done = true;
            return TranslationEffect::FinalTranslationFix(self.target);
        }

        TranslationEffect::LinearVelocity(self.direction * speed)
    }

    fn has_reached_destination(&mut self, transform: Transform, speed: f32) -> bool {
        let remaining_distance_reached_threshold = speed * LINEAR_THRESHOLD_RATIO;

        let remaining_distance = transform.translation.distance(self.target);
        let last_remaining_distance = self.state.last_remaining_distance;
        self.state.last_remaining_distance = remaining_distance;

        let has_reached_destination = remaining_distance <= remaining_distance_reached_threshold;
        let has_gone_past_destination = remaining_distance > last_remaining_distance;

        has_reached_destination || has_gone_past_destination
    }

    fn is_in_facing_threshold(&self, transform: Transform, rotation_speed: f32) -> bool {
        let current_direction = transform.forward();
        let angle_between = current_direction.angle_between(self.direction);
        let angle_between_threshold = rotation_speed * ANGULAR_THRESHOLD_RATIO;
        angle_between <= angle_between_threshold
    }

    fn get_needed_angular_velocity(&self, transform: Transform) -> Vec3 {
        let current_direction = transform.forward();

        #[allow(clippy::let_and_return)]
        let rotation_axis = current_direction.cross(self.direction).normalize();
        rotation_axis
    }
}
