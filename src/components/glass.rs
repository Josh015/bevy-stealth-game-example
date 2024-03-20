use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

/// A wall that can be shattered by the Shockwave Cannon.
#[derive(Clone, Component, Debug, Default)]
pub struct Glass;

/// [`Glass`] entity AI states.
#[derive(
    Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect,
)]
#[component(storage = "SparseSet")]
pub enum GlassState {
    #[default]
    Solid,
    Shattered,
}

/// Required components for a [`Glass`] entity.
#[derive(Bundle)]
pub struct GlassBundle {
    pub glass: Glass,
    pub actions_bundle: ActionsBundle,
    pub state_machine: StateMachine,
    pub glass_state: GlassState,
}

impl Default for GlassBundle {
    fn default() -> Self {
        Self {
            glass: Glass,
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default(),
            glass_state: GlassState::default(),
        }
    }
}

// TODO: Use a state machine and actions to remove its Barrier and spawn a
// particle effect.
