use bevy::prelude::Vec3;

pub struct Target {
    target: Vec3,
    direction: Vec3,
    rotation_done: bool,
    destination_reached: bool
}


pub enum Destination {
    Target(Target),
    Reached
}


