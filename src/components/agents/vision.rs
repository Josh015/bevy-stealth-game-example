use bevy::{app::prelude::*, ecs::prelude::*};

pub(super) struct VisionPlugin;

impl Plugin for VisionPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins((movement::MovementPlugin,));
    }
}

/// A vision cone that can trigger a [`SawPlayerEvent`](crate::events::SawPlayerEvent).
#[derive(Clone, Component, Debug, Default)]
pub struct Vision {
    pub distance: f32,
    pub fov: f32,
}
