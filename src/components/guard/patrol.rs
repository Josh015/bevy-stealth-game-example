use bevy::prelude::*;
use bevy_sequential_actions::*;
use seldom_state::prelude::*;
use std::time::Duration;

use crate::prelude::*;

pub(super) struct PatrolPlugin;

impl Plugin for PatrolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, patrol_location.in_set(StoppedWhenPausedSet));
    }
}

const IDLE_DELAY: Duration = Duration::from_millis(1_000);

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct Patrol;

fn patrol_location(
    mut commands: Commands,
    query: Query<Entity, (With<Guard>, Added<Patrol>)>,
) {
    for entity in &query {
        // Repeat Sequence (forever):
        //   <generate for all patrol points>:
        //     Move to next point.
        //     Turn to face next point.
        //     Wait.

        // <generate path back to guard location>
        //   Move to next point.
        // Turn to face guard direction.
        // Start "idle" animation (blocking, repeating).

        commands.actions(entity).add_many(actions![
            AnimationAction::non_blocking("idle"),
            WaitAction::new(IDLE_DELAY),
            |agent: Entity, world: &mut World| -> bool {
                world.entity_mut(agent).insert(Done::Success);
                true
            },
        ]);
    }
}

// TODO: Takes an optional level script at spawn time?
// If none is provided, use default that returns to starting location and facing direction?
