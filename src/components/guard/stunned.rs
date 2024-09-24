use bevy::prelude::*;

use crate::prelude::*;

pub(super) struct StunnedPlugin;

impl Plugin for StunnedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, stun_response.in_set(StoppedWhenPausedSet));
    }
}

/// A [`Guard`] that's able to be stunned.
#[derive(Clone, Component, Debug, Default)]
pub struct Stunnable;

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct Stunned;

fn stun_response(
    mut commands: Commands,
    query: Query<Entity, (With<Guard>, Added<Stunned>)>,
) {
    for entity in &query {
        // Parallel Actions:
        //   Play "Stunned" sound (blocking, once).
        //   Play "Stunned" animation (blocking, once).
        // Wait (stun duration).
        // Play "Recovering" animation (blocking, once).
        // Done.
    }
}
