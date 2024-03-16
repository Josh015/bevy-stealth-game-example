use bevy::prelude::*;

pub mod moving;
pub mod turning;

/// Movement speed in meters/second.
#[derive(Clone, Component, Debug, Default)]
pub struct MovingSpeed(pub f32);

/// Turning speed in radians/second.
#[derive(Clone, Component, Debug, Default)]
pub struct TurningSpeed(pub f32);

/// Required components for movement.
#[derive(Bundle, Clone, Debug, Default)]
pub struct MovementBundle {
    pub moving_speed: MovingSpeed,
    pub turning_speed: TurningSpeed,
}

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((moving::MovingPlugin, turning::TurningPlugin));
    }
}
