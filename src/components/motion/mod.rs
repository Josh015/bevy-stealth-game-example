use bevy::prelude::*;

mod rotating;
mod velocity;

pub use rotating::*;
pub use velocity::*;

/// Moving speed in `meters/second`.
#[derive(Clone, Component, Debug, Default)]
pub struct MovingSpeed(pub f32);

/// Turning speed in `radians/second`.
#[derive(Clone, Component, Debug, Default)]
pub struct TurningSpeed(pub f32);

/// Required components for movement.
#[derive(Bundle, Clone, Debug, Default)]
pub struct MotionBundle {
    pub moving_speed: MovingSpeed,
    pub turning_speed: TurningSpeed,
}

pub(super) struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RotatingPlugin, VelocityPlugin));
    }
}
