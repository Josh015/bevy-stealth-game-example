use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

/// Required components for a [`FloorSwitch`] entity.
#[derive(Bundle)]
pub struct FloorSwitchBundle {
    pub floor_switch: FloorSwitch,
    pub actions_bundle: ActionsBundle,
    pub state_machine: StateMachine,
    pub switched_off: SwitchedOff,
}

impl Default for FloorSwitchBundle {
    fn default() -> Self {
        // TODO: Use a state machine and actions to make it animate, signal open doors,
        // delay, and then signal doors to close them again.

        Self {
            floor_switch: FloorSwitch,
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default(),
            switched_off: SwitchedOff,
        }
    }
}

/// A switch on the floor that the player can step on to temporarily activate.
#[derive(Clone, Component, Debug, Default)]
pub struct FloorSwitch;

/// [`FloorSwitch`] default state where it's off.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct SwitchedOff;

/// [`FloorSwitch`] state where it's on.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct SwitchedOn;

/// [`FloorSwitch`] state where it's resetting.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct ResettingSwitch;

/// [`FloorSwitch`] state where it's already pressed down.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct PressingSwitch;
