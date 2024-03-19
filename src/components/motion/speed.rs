use bevy::ecs::prelude::*;

/// Directional speed in `meters/second`.
#[derive(Clone, Component, Debug, Default)]
pub struct Speed(pub f32);

/// Angular speed in `radians/second`.
#[derive(Clone, Component, Debug, Default)]
pub struct AngularSpeed(pub f32);

/// Required components for movement.
#[derive(Bundle, Clone, Debug, Default)]
pub struct SpeedBundle {
    pub speed: Speed,
    pub angular_speed: AngularSpeed,
}
