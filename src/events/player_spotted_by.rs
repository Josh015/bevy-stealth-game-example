use bevy::ecs::prelude::*;

// Fired when a vision cone intersects the visible player.
pub struct PlayerSpottedBy(Entity);

// TODO: Guards will pass their own ID, but cameras will pass ID of the guard
// closest to player!
