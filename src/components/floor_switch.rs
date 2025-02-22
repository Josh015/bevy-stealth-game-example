use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

/// A switch on the floor that the player can step on to temporarily activate.
#[derive(Clone, Component, Debug, Default)]
#[require(FloorSwitchState, SequentialActions, StateMachine)]
pub struct FloorSwitch;

/// [`FloorSwitch`] current state.
#[derive(Clone, Component, Copy, Default, Reflect)]
#[component(storage = "SparseSet")]
pub enum FloorSwitchState {
    #[default]
    Off,
    On,
}

/// [`FloorSwitch`] transition states.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub enum FloorSwitchTransitionState {
    Pressing,
    Resetting,
}
