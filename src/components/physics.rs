use bevy::{app::prelude::*, ecs::prelude::*};

pub(super) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins((movement::MovementPlugin,));
    }
}

/// 2D Circular collision primitive.
#[derive(Clone, Component, Debug, Default)]
pub struct Physics {
    pub radius: f32,
}
