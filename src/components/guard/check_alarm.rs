use std::time::Duration;

use bevy::prelude::*;
use bevy_sequential_actions::*;
use rand::prelude::*;
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
        let mut sequential_actions = commands.actions(entity);

        match check_alarm {
            Surprised => {
                sequential_actions.add_many(actions![
                    ParallelActions::new(actions![
                        AnimationAction::new("alert"),
                        EmoteAction::new("alert"),
                    ]),
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(GoTo);
                        true
                    },
                ]);
            },
            GoTo => {
                // <path to player last seen position by camera.>

                sequential_actions.add(
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(Searching);
                        true
                    },
                );
            },
            Searching => {
                let mut rng = SmallRng::from_entropy();

                for _ in 0..2 {
                    let mut random_vector = Vec3::ZERO;
                    random_vector.x = rng.gen_range(-1.0..=1.0);
                    random_vector.z = rng.gen_range(-1.0..=1.0);

                    let random_direction =
                        Dir3::new_unchecked(random_vector.normalize_or_zero());

                    sequential_actions.add_many(actions![
                        FaceDirectionAction::new(random_direction),
                        WaitAction::new(Duration::from_millis(1500)),
                    ]);
                }

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
