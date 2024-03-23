use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

/// Required components for a [`Door`] entity.
#[derive(Bundle)]
pub struct DoorBundle {
    pub door: Door,
    pub actions_bundle: ActionsBundle,
    pub state_machine: StateMachine,
    pub door_state: DoorState,
}

impl Default for DoorBundle {
    fn default() -> Self {
        // TODO: Use a state machine and actions to make it animate open and
        // remove its Barrier, but then restore Barrier and close once
        // signaled by floor switch.

        Self {
            door: Door,
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default(),
            door_state: DoorState::Closed,
        }
    }
}

/// A barrier that can be opened/closed.
#[derive(Clone, Component, Debug, Default)]
pub struct Door;

/// [`Door`] current state.
#[derive(Clone, Component, Copy, Default, Reflect)]
#[component(storage = "SparseSet")]
pub enum DoorState {
    #[default]
    Closed,
    Open,
}

/// [`Door`] transition states.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub enum DoorTransitionState {
    Closing,
    Opening,
}
