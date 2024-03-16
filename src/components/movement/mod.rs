use bevy::prelude::*;

pub mod moving;
pub mod rotating;

// Controls how fast an entity can move.
#[derive(Clone, Component, Debug, Default)]
pub struct Movement;
// speed: f32

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((moving::MovingPlugin, rotating::RotatingPlugin));
    }
}
