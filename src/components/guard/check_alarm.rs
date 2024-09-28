use bevy::prelude::*;
use bevy_sequential_actions::*;
use seldom_state::prelude::*;

use crate::prelude::*;

pub(super) struct CheckAlarmPlugin;

impl Plugin for CheckAlarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_alarm.in_set(StoppedWhenPausedSet));
    }
}

#[derive(Clone, Component, Copy, Default, Reflect)]
#[component(storage = "SparseSet")]
pub enum CheckAlarm {
    #[default]
    Surprised,
    GoTo,
    Searching,
}

fn check_alarm(
    mut commands: Commands,
    query: Query<(Entity, &CheckAlarm), (With<Guard>, Changed<CheckAlarm>)>,
) {
    use CheckAlarm::*;

    for (entity, check_alarm) in &query {
        match check_alarm {
            Surprised => {
                let mut sequential_actions = commands.actions(entity);

                // Turn to face direction of player.
                // Parallel Actions:
                //   Play "Surprised" animation (blocking, once).
                //   Emit "!" emote (blocking).
                // Emit emphasized "!" emote (non-blocking).

                sequential_actions.add(
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(GoTo);
                        true
                    },
                );
            },
            GoTo => {
                let mut sequential_actions = commands.actions(entity);

                // <path to player last known location.>

                sequential_actions.add(
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(Searching);
                        true
                    },
                );
            },
            Searching => {
                let mut sequential_actions = commands.actions(entity);

                // Turn to random direction.
                // Wait.
                // Turn to random direction.
                // Wait.

                sequential_actions.add(
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(Done::Failure);
                        true
                    },
                );
            },
        }
    }
}
