use bevy::prelude::*;

/// Linear speed in `meters/second`.
#[derive(Clone, Component, Debug)]
pub struct MoveSpeed(pub f32);

impl Default for MoveSpeed {
    fn default() -> Self { Self(1.0) }
}

/// Angular speed in `radians/second`.
#[derive(Clone, Component, Debug)]
pub struct RotateSpeed(pub f32);

impl Default for RotateSpeed {
    fn default() -> Self { Self(std::f32::consts::TAU) }
}
