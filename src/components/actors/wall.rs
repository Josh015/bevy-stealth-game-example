use bevy::ecs::prelude::*;

/// A wall that prevents entities from moving through it.
#[derive(Clone, Component, Debug, Default)]
pub struct Wall;
