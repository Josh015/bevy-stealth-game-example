use bevy::prelude::*;

mod angular_velocity;
mod velocity;

pub use angular_velocity::*;
pub use velocity::*;

pub(super) struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AngularVelocityPlugin, VelocityPlugin));
    }
}

/// Directional speed in `meters/second`.
#[derive(Clone, Component, Debug, Default)]
pub struct Speed(pub f32);

/// Angular speed in `radians/second`.
#[derive(Clone, Component, Debug, Default)]
pub struct AngularSpeed(pub f32);

/// Required components for movement.
#[derive(Bundle, Clone, Debug, Default)]
pub struct MotionBundle {
    pub speed: Speed,
    pub angular_speed: AngularSpeed,
}
