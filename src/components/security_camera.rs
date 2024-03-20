use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

/// Required components for a [`SecurityCamera`] entity.
#[derive(Bundle)]
pub struct SecurityCameraBundle {
    pub security_camera: SecurityCamera,
    pub actions_bundle: ActionsBundle,
    pub state_machine: StateMachine,
    pub security_camera_state: SecurityCameraState,
}

impl Default for SecurityCameraBundle {
    fn default() -> Self {
        // TODO: Use a state machine and actions to make it move back and forth.

        Self {
            security_camera: SecurityCamera,
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default(),
            security_camera_state: SecurityCameraState::PanningLeft,
        }
    }
}

/// Designates a security camera entity that can raise alarm events when it
/// sees the player.
#[derive(Clone, Component, Debug, Default)]
pub struct SecurityCamera;

/// [`SecurityCamera`] current state.
#[derive(Clone, Component, Copy, Default, Reflect)]
#[component(storage = "SparseSet")]
pub enum SecurityCameraState {
    #[default]
    PanningRight,
    PanningLeft,
}
