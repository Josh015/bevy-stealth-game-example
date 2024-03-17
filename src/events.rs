use bevy::prelude::*;

pub(super) struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SawPlayerEvent>()
            .add_event::<HeardSoundEvent>()
            .add_event::<StunnedEvent>();
    }
}

/// Fired when an enemy vision cone intersects the visible player.
#[derive(Clone, Component, Debug, Event)]
pub struct SawPlayerEvent {
    pub witness: Entity,
}

/// Fired when a sound wave collides with an enemy's hearing radius.
#[derive(Clone, Component, Debug, Event)]
pub struct HeardSoundEvent {
    pub sound_position: Vec3,
}

/// Fired when an enemy is stunned.
#[derive(Clone, Component, Debug, Event)]
pub struct StunnedEvent {
    pub victim: Entity,
}

// UiMessageEvent
// position: Top | Bottom
// text: String

// Pops up a message in the UI at either top/bottom center. Top is for level
// names or announcing pickups and stacks messages based on the order they were
// sent. Bottom is for story/progress messages from other characters.Fade them
// out over time.

// Collision
// Two entities collided.
// TODO: Should this be handled by a proper physics engine, or is it simple
// enough to handle ourselves?
