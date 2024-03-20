use bevy::ecs::prelude::*;

/// A barrier that prevents physical entities from moving through it.
#[derive(Clone, Component, Debug, Default)]
pub struct Barrier;

/// A wall that can block guard's vision cones.
#[derive(Clone, Component, Debug, Default)]
pub struct BlocksVision;

/// A wall that can deflect sound waves.
#[derive(Clone, Component, Debug, Default)]
pub struct DeflectsSounds;

/// A door that can be opened/closed.
#[derive(Clone, Component, Debug, Default)]
pub struct Door;

/// A wall that can be shattered by the Shockwave Cannon.
#[derive(Clone, Component, Debug, Default)]
pub struct Shatterable;
