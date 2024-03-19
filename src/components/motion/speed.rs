use bevy::ecs::prelude::*;

/// Directional speed in `meters/second`.
#[derive(Clone, Component, Debug)]
pub struct Speed(pub f32);

impl Default for Speed {
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

/// Required components for movement.
#[derive(Bundle, Clone, Debug, Default)]
pub struct SpeedBundle {
    pub speed: Speed,
    pub angular_speed: AngularSpeed,
}
