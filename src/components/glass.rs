use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

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
        // TODO: Use a state machine and actions to remove its Barrier and spawn
        // a particle effect.

        Self {
            glass: Glass,
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default(),
            glass_state: GlassState::Solid,
        }
    }
}

/// A wall that can be shattered by the Shockwave Cannon.
#[derive(Clone, Component, Debug, Default)]
pub struct Glass;

/// [`Glass`] current state.
#[derive(Clone, Component, Copy, Default, Reflect)]
#[component(storage = "SparseSet")]
pub enum GlassState {
    #[default]
    Solid,
    Shattered,
}

/// [`Glass`] state where it's being shattered by the player.
#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct Shattering;
