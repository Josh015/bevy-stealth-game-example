use bevy::prelude::*;

mod mover;
mod velocity;

pub use mover::*;
pub use velocity::*;

pub(super) struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MoverPlugin, VelocityPlugin));
    }
}
