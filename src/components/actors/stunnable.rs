use bevy::ecs::prelude::*;

/// A guard that's able to be stunned by the Stun Gun.
#[derive(Clone, Component, Debug, Default)]
pub struct Stunnable;
