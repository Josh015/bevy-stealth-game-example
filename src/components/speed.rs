use bevy::prelude::*;

/// Required components for systems that use speed.
#[derive(Bundle, Default)]
pub struct SpeedBundle {
    pub linear_speed: LinearSpeed,
    pub angular_speed: AngularSpeed,
}

/// Linear speed in `meters/second`.
#[derive(Clone, Component, Debug)]
pub struct LinearSpeed(pub f32);

impl Default for LinearSpeed {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Angular speed in `radians/second`.
#[derive(Clone, Component, Debug)]
pub struct AngularSpeed(pub f32);

impl Default for AngularSpeed {
    fn default() -> Self {
        Self(std::f32::consts::TAU)
    }
}
