use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

/// A wall that can be shattered by the Shockwave Cannon.
#[derive(Clone, Component, Debug, Default)]
#[require(GlassState, SequentialActions, StateMachine)]
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
