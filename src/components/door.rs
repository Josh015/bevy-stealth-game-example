use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

/// A barrier that can be opened/closed.
#[derive(Clone, Component, Debug, Default)]
#[require(DoorState, SequentialActions, StateMachine)]
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
