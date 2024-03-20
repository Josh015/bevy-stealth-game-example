use bevy::ecs::prelude::*;

/// A wall that can be shattered by the Shockwave Cannon.
#[derive(Clone, Component, Debug, Default)]
pub struct Glass;

// TODO: Use a state machine and actions to remove its Barrier and spawn a
// particle effect.
