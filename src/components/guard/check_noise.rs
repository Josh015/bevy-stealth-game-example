use bevy::prelude::*;

use crate::prelude::*;

pub(super) struct CheckNoisePlugin;

impl Plugin for CheckNoisePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_alarm.in_set(StoppedWhenPausedSet));
    }
}

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct CheckNoise;

fn check_alarm(
    mut commands: Commands,
    query: Query<Entity, (With<Guard>, Added<CheckNoise>)>,
) {
    for entity in &query {
        // Parallel Actions:
        //   Play "What the?" sound (blocking, once).
        //   Emit "?" emote (blocking).
        //   Turn to face direction of sound.
        // Play "Searching" animation (blocking, once).
        // Done.
    }
}
