use bevy::ecs::prelude::*;

/// A wall that can block guard's vision cones.
#[derive(Clone, Component, Debug, Default)]
pub struct BlocksVision;
