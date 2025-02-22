use bevy::prelude::*;

/// Linear speed in `meters/second`.
#[derive(Clone, Component, Debug)]
pub struct LinearSpeed(pub f32);

impl Default for LinearSpeed {
    fn default() -> Self { Self(1.0) }
}

/// Angular speed in `radians/second`.
#[derive(Clone, Component, Debug)]
pub struct AngularSpeed(pub f32);

impl Default for AngularSpeed {
    fn default() -> Self { Self(std::f32::consts::TAU) }
}
