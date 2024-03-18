use bevy::prelude::*;

mod moving_to;
mod rotating;
mod translating;
mod turning_to;

pub use moving_to::*;
pub use rotating::*;
pub use translating::*;
pub use turning_to::*;

/// Moving speed in `meters/second`.
#[derive(Clone, Component, Debug, Default)]
pub struct MovingSpeed(pub f32);

/// Turning speed in `radians/second`.
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
        app.add_plugins((
            MovingToPlugin,
            RotatingPlugin,
            TranslatingPlugin,
            TurningPlugin,
        ));
    }
}
