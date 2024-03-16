use bevy::ecs::prelude::*;

// Allows an entity to keep track of where it started and where it can return to
// after chasing the player.
pub struct StartLocation {
    pub spawn_point: Entity,
}
