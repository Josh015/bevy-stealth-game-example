use bevy::ecs::prelude::*;

/// A barrier that prevents physical entities from moving through it.
#[derive(Clone, Component, Debug, Default)]
pub struct Barrier;
