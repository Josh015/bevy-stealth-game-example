use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

/// Required components for a [`SecurityCamera`] entity.
#[derive(Bundle)]
pub struct SecurityCameraBundle {
    pub security_camera: SecurityCamera,
    pub actions_bundle: ActionsBundle,
    pub state_machine: StateMachine,
    pub panning_right: PanningRight,
}

impl Default for SecurityCameraBundle {
    fn default() -> Self {
        // TODO: Use a state machine and actions to make it move back and forth.

        Self {
            security_camera: SecurityCamera,
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default(),
            panning_right: PanningRight,
        }
    }
}

/// Designates a security camera entity.
#[derive(Clone, Component, Debug, Default)]
pub struct SecurityCamera;

/// [`SecurityCamera`] state where it's panning to the right.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct PanningRight;

/// [`SecurityCamera`] state where it's panning to the left.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct PanningLeft;
