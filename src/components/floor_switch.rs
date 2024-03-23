use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

/// Required components for a [`FloorSwitch`] entity.
#[derive(Bundle)]
pub struct FloorSwitchBundle {
    pub floor_switch: FloorSwitch,
    pub actions_bundle: ActionsBundle,
    pub state_machine: StateMachine,
    pub switch_state: SwitchState,
}

impl Default for FloorSwitchBundle {
    fn default() -> Self {
        // TODO: Use a state machine and actions to make it animate, signal open
        // doors, delay, and then signal doors to close them again.

        Self {
            floor_switch: FloorSwitch,
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default(),
            switch_state: SwitchState::Off,
        }
    }
}

/// A switch on the floor that the player can step on to temporarily activate.
#[derive(Clone, Component, Debug, Default)]
pub struct FloorSwitch;

/// [`FloorSwitch`] current state.
#[derive(Clone, Component, Copy, Default, Reflect)]
#[component(storage = "SparseSet")]
pub enum SwitchState {
    #[default]
    On,
    Off,
}

/// [`FloorSwitch`] transition states.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub enum SwitchTransitionState {
    Pressing,
    Resetting,
}
