use bevy::ecs::prelude::*;

/// Attaches a drop shadow to an entity to show their distance from the ground.
#[derive(Clone, Component, Debug, Default)]
pub struct DropShadow;
