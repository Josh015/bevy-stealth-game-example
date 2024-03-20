use bevy::ecs::prelude::*;

/// A door that can be opened/closed.
#[derive(Clone, Component, Debug, Default)]
pub struct Door;

// TODO: Use a state machine and actions to make it animate open and remove its
// Barrier, but then restore Barrier and close once signaled by floor switch.

//   States:
//     Opening
//     Opened
//     Closing
//     Closed
//
