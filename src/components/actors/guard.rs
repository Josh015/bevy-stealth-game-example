use bevy::{app::prelude::*, ecs::prelude::*};

pub(super) struct GuardPlugin;

impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins((movement::MovementPlugin,));
    }
}

/// Designates a guard entity.
#[derive(Clone, Component, Debug, Default)]
pub struct Guard;

// TODO: Retrieve starting location position and direction from marked entity.
// TODO: All nodes of a specific patrol route children of one encompassing
// entity?
