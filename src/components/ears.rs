use bevy::{app::prelude::*, ecs::prelude::*};

pub(super) struct EarsPlugin;

impl Plugin for EarsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins((movement::MovementPlugin,));
    }
}

/// Entities that can hear and respond to sound waves.
#[derive(Clone, Component, Debug, Default)]
pub struct Ears {
    pub radius: f32,
}
