use bevy::ecs::entity::Entity;

/// Designates a Guard entity and what they're guarding.
#[derive(Default)]
pub enum Guard {
    #[default]
    StartLocation,
    PatrolRoute(Entity),
}

// TODO: Retrieve starting location position and direction from marked entity.
// TODO: All nodes of a specific patrol route children of one encompassing
// entity?
