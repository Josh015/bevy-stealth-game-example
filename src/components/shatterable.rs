use bevy::ecs::prelude::*;

/// A wall that can be shattered by the Shockwave Cannon.
#[derive(Clone, Component, Debug, Default)]
pub struct Shatterable;
