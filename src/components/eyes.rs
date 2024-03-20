use bevy::{app::prelude::*, ecs::prelude::*};

pub(super) struct EyesPlugin;

impl Plugin for EyesPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins((movement::MovementPlugin,));
    }
}

/// Entities that can see the player.
#[derive(Clone, Component, Debug, Default)]
pub struct Eyes {
    pub distance: f32,
    pub fov: f32,
}
