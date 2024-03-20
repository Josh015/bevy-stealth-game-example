use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

/// Required components for a [`Door`] entity.
#[derive(Bundle)]
pub struct DoorBundle {
    pub door: Door,
    pub actions_bundle: ActionsBundle,
    pub state_machine: StateMachine,
    pub closed: Closed,
}

impl Default for DoorBundle {
    fn default() -> Self {
        // TODO: Use a state machine and actions to make it animate open and remove its
        // Barrier, but then restore Barrier and close once signaled by floor switch.

        Self {
            door: Door,
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default(),
            closed: Closed,
        }
    }
}

/// A door that can be opened/closed.
#[derive(Clone, Component, Debug, Default)]
pub struct Door;

/// [`Door`] state where it's blocking passage.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct Closed;

/// [`Door`] state where it's blocking passage and closing.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct Closing;

/// [`Door`] state where it's not blocking passage.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct Open;

/// [`Door`] state where it's blocking passage and opening.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct Opening;
