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

/// Required components for a [`Mover`] entity.
#[derive(Bundle, Clone, Debug)]
pub struct GuardBundle {
    pub guard: Guard,
}

impl Default for GuardBundle {
    fn default() -> Self {
        // TODO: Set up state machine here.
        // Check for special components in the transition checks for those states.

        Self { guard: Guard }
    }
}

// TODO: Retrieve starting location position and direction from marked entity.
// TODO: All nodes of a specific patrol route children of one encompassing
// entity?
