use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

/// Designates a security camera entity that can raise alarm events when it
/// sees the player.
#[derive(Clone, Component, Debug, Default)]
#[require(SecurityCameraState, SequentialActions, StateMachine)]
pub struct SecurityCamera;

/// [`SecurityCamera`] current state.
#[derive(Clone, Component, Copy, Default, Reflect)]
#[component(storage = "SparseSet")]
pub enum SecurityCameraState {
    #[default]
    PanningRight,
    PanningLeft,
}
