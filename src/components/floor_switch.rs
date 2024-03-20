use bevy::ecs::prelude::*;

/// Designates a switch that the player can step on to temporarily activate.
#[derive(Clone, Component, Debug, Default)]
pub struct FloorSwitch;

// TODO: Use a state machine and actions to make it animate, signal open doors,
// delay, and then signal doors to close them again.
