use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        //app.add_event::<HeardNoiseEvent>();
    }
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
