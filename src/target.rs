use bevy::prelude::{Quat, Transform, Vec3};

pub enum Status {
    OnGoing,
    Paused,
}

pub struct Target {
    target: Vec3,
    direction: Vec3,
    last_distance_to_target: f32,
    status: Status,
    rotation_done: bool,
    destination_reached: bool,
}

const LINEAR_THRESHOLD_RATIO: f32 = 120.0;
const ANGULAR_THRESHOLD_RATIO: f32 = 60.0;

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
    pub fn new(from: Vec3, target: Vec3) -> Self {
        if (target - from).length() == 0.0 {
            Self {
                target,
                direction: Vec3::default(),
                last_distance_to_target: 0.0,
                status: Status::OnGoing,
                rotation_done: true,
                destination_reached: true,
            }
        } else {
            Self {
                target,
                direction: (target - from).normalize(),
                last_distance_to_target: from.distance(target),
                status: Status::OnGoing,
                rotation_done: false,
                destination_reached: false,
            }
        }
    }

    pub fn pause(&mut self) {
        self.status = Status::Paused;
    }

    pub fn resume(&mut self) {
        self.status = Status::OnGoing;
    }

    pub fn set_destination_reached(&mut self) {
        self.destination_reached = true;
    }

    pub fn get_rotation_effect(
        &mut self,
        transform: Transform,
        rotation_speed: f32,
    ) -> RotationEffect {
        if self.rotation_done {
            return RotationEffect::RotationDone;
        }

        if self.is_in_facing_threshold(transform, rotation_speed) {
            self.rotation_done = true;
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
        if self.destination_reached {
            return TranslationEffect::DestinationReached;
        }

        if self.has_reached_destination(transform, speed) {
            self.destination_reached = true;
            return TranslationEffect::FinalTranslationFix(self.target);
        }

        TranslationEffect::LinearVelocity(self.direction * speed)
    }

    fn has_reached_destination(&mut self, transform: Transform, speed: f32) -> bool {
        let current_distance_to_target = transform.translation.distance(self.target);

        let destination_reached_threshold = speed / LINEAR_THRESHOLD_RATIO;
        let has_reached_destination = current_distance_to_target < destination_reached_threshold;

        let has_gone_past_destination = current_distance_to_target > self.last_distance_to_target;
        self.last_distance_to_target = current_distance_to_target;

        has_reached_destination || has_gone_past_destination
    }

    fn is_in_facing_threshold(&self, transform: Transform, rotation_speed: f32) -> bool {
        let current_direction = transform.forward();
        let angle_between = current_direction.angle_between(self.direction);

        let angle_between_threshold = rotation_speed / ANGULAR_THRESHOLD_RATIO;

        angle_between <= angle_between_threshold
    }

    fn get_needed_angular_velocity(&self, transform: Transform) -> Vec3 {
        let current_direction = transform.forward();

        #[allow(clippy::let_and_return)]
        let rotation_axis = current_direction.cross(self.direction).normalize();
        rotation_axis
    }
}
