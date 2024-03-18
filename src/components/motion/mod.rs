use bevy::prelude::*;

mod rotating;
mod velocity;

pub use rotating::*;
pub use velocity::*;

pub(super) struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RotatingPlugin, VelocityPlugin));
    }
}

/// Directional speed in `meters/second`.
#[derive(Clone, Component, Debug, Default)]
pub struct Speed(pub f32);

/// Rotational speed in `radians/second`.
#[derive(Clone, Component, Debug, Default)]
pub struct RotationalSpeed(pub f32);

/// Required components for movement.
#[derive(Bundle, Clone, Debug, Default)]
pub struct MotionBundle {
    pub speed: Speed,
    pub rotational_speed: RotationalSpeed,
}
