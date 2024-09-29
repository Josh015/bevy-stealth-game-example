use bevy::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;
use rand::prelude::*;
use seldom_state::prelude::*;
use std::time::Duration;

use crate::prelude::*;

pub(super) struct GuardPlugin;

impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (guarding, trigger_game_over_on_player_collision)
                .in_set(StoppedWhenPausedSet),
        );
    }
}

/// Required components for a [`Guard`] entity.
#[derive(Bundle)]
pub struct GuardBundle {
    pub guard: Guard,
    pub patrol_start_location: PatrolStartLocation,
    pub actions_bundle: ActionsBundle,
    pub state_machine: StateMachine,
}

impl GuardBundle {
    pub fn with_start_location(transform: Transform) -> Self {
        use Guard::*;

        Self {
            guard: Guard::Patrol,
            patrol_start_location: PatrolStartLocation::new(transform),
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default()
                .trans::<AnyState, _>(done(None), Patrol)
                .trans::<AnyState, _>(stunned, Stunned)
                .trans_builder(saw_player, |guard, player_location| match guard
                {
                    Patrol | CheckNoise(_) | SearchNearAlarm | GoToAlarm(_)
                    | Alarmed(_) => Some(SawPlayer(player_location)),
                    LostPlayer => Some(ChasePlayer(player_location)),
                    _ => None,
                })
                .trans_builder(
                    heard_alarm,
                    |guard, player_location| match guard {
                        Patrol | CheckNoise(_) | LostPlayer => {
                            Some(Alarmed(player_location))
                        },
                        SearchNearAlarm => Some(GoToAlarm(player_location)),
                        _ => None,
                    },
                )
                .trans_builder(
                    heard_noise,
                    |guard, noise_direction| match guard {
                        Patrol | CheckNoise(_) | SearchNearAlarm
                        | LostPlayer => Some(CheckNoise(noise_direction)),
                        _ => None,
                    },
                ),
        }
    }
}

#[derive(Clone, Component, Debug, Default, new)]
pub struct PatrolStartLocation {
    transform: Transform,
}

/// Designates a guard entity and represents its current state.
#[derive(Clone, Component, Copy, Default, Reflect)]
#[component(storage = "SparseSet")]
pub enum Guard {
    #[default]
    Patrol,
    CheckNoise(Dir3),
    Alarmed(Vec3),
    GoToAlarm(Vec3),
    SearchNearAlarm,
    SawPlayer(Vec3),
    ChasePlayer(Vec3),
    LostPlayer,
    Stunned,
}

/// A [`Guard`] that's able to be stunned.
#[derive(Clone, Component, Debug, Default)]
pub struct Stunnable;

/// A [`Guard`] that can see the player.
#[derive(Clone, Component, Debug, Default)]
pub struct Vision {
    pub distance: f32,
    pub fov: f32,
}

/// A [`Guard`] that can hear and respond to sound waves.
#[derive(Clone, Component, Debug, Default)]
pub struct Hearing {
    pub radius: f32,
}

fn stunned(In(entity): In<Entity>, query: Query<Has<Stunnable>>) -> bool {
    let is_stunnable = query.get(entity).unwrap();

    false
}

fn saw_player(
    In(entity): In<Entity>,
    query: Query<Has<Vision>>,
    targets: Query<&Transform, With<Target>>,
) -> Option<Vec3> {
    // TODO: Use parent observer to bubble event up from child component.
    let has_vision = query.get(entity).unwrap();

    if !has_vision {
        return None;
    }

    let Ok(player_transform) = targets.get_single() else {
        return None;
    };

    Some(player_transform.translation)
}

fn heard_alarm(
    In(entity): In<Entity>,
    query: Query<Has<Hearing>>,
) -> Option<Vec3> {
    let has_hearing = query.get(entity).unwrap();

    // TODO: Use parent observer to bubble event up from child component.

    None
}

fn heard_noise(
    In(entity): In<Entity>,
    query: Query<Has<Hearing>>,
) -> Option<Dir3> {
    let has_hearing = query.get(entity).unwrap();

    // TODO: Use parent observer to bubble event up from child component.

    None
}

fn trigger_game_over_on_player_collision(
    mut commands: Commands,
    guard_query: Query<&Transform, With<Guard>>,
    player_query: Query<(Entity, &Transform), With<Target>>,
) {
    for transform in &guard_query {
        // TODO: Actual Player entity will work with just .single().
        let Ok((player_entity, player_transform)) = player_query.get_single()
        else {
            continue;
        };

        if transform.translation.distance(player_transform.translation) < 0.1 {
            commands.entity(player_entity).despawn_recursive();
        }
    }
}

fn guarding(
    mut commands: Commands,
    query: Query<
        (Entity, &Transform, &Guard, &PatrolStartLocation),
        Changed<Guard>,
    >,
) {
    use Guard::*;

    for (entity, transform, guard, patrol_start_location) in &query {
        let mut sequential_actions = commands.actions(entity);

        sequential_actions.clear();

        match guard {
            Patrol => {
                // TODO: Takes an optional level script at spawn time?
                // If none is provided, use default that returns to starting location and facing direction?

                sequential_actions.add_many(actions![
                    MoveToAction::new(
                        patrol_start_location.transform.translation
                    ),
                    FaceDirectionAction::new(
                        -patrol_start_location.transform.forward()
                    ),
                    AnimationAction::new("idle"),
                ]);
            },
            CheckNoise(noise_direction) => {
                sequential_actions.add_many(actions![
                    ParallelActions::new(actions![
                        SoundAction::new("distracted"),
                        EmoteAction::new("sound"),
                        FaceDirectionAction::new(*noise_direction),
                    ]),
                    AnimationAction::new("confused"),
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(Done::Failure);
                        true
                    },
                ]);
            },
            Alarmed(player_location) => {
                let player_location = player_location.clone();

                sequential_actions.add_many(actions![
                    ParallelActions::new(actions![
                        AnimationAction::new("alert"),
                        EmoteAction::new("alert"),
                    ]),
                    move |agent: Entity, world: &mut World| -> bool {
                        world
                            .entity_mut(agent)
                            .insert(GoToAlarm(player_location));
                        true
                    },
                ]);
            },
            GoToAlarm(player_location) => {
                commands.actions(entity).add_many(actions![
                    MoveToAction::new(*player_location),
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(SearchNearAlarm);
                        true
                    },
                ]);
            },
            SearchNearAlarm => {
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
            SawPlayer(player_location) => {
                let player_location = player_location.clone();
                let guard_position = transform.translation;
                let face_player_direction = Dir3::new_unchecked(
                    (player_location - guard_position).normalize_or_zero(),
                );

                sequential_actions.add_many(actions![
                    FaceDirectionAction::new(face_player_direction),
                    ParallelActions::new(actions![
                        SoundAction::new("alerted"),
                        AnimationAction::new("alert"),
                        EmoteAction::new("alert"),
                    ]),
                    move |agent: Entity, world: &mut World| -> bool {
                        world
                            .entity_mut(agent)
                            .insert(ChasePlayer(player_location));
                        true
                    },
                ]);
            },
            ChasePlayer(player_location) => {
                commands.actions(entity).add_many(actions![
                    EmoteAction::non_blocking("chase"),
                    MoveToAction::new(*player_location),
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(SearchNearAlarm);
                        true
                    },
                ]);
            },
            LostPlayer => {
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

                sequential_actions.add_many(actions![
                    ParallelActions::new(actions![
                        AnimationAction::new("frustrated"),
                        EmoteAction::new("frustrated"),
                    ]),
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(Done::Failure);
                        true
                    },
                ]);
            },
            Stunned => {
                sequential_actions.add_many(actions![
                    ParallelActions::new(actions![
                        AnimationAction::new("stun"),
                        SoundAction::new("stun"),
                    ]),
                    WaitAction::new(Duration::from_secs(3)),
                    AnimationAction::new("unstun"),
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(Done::Success);
                        true
                    },
                ]);
            },
        }
    }
}

// TODO: Remove this later.
#[derive(Component)]
pub struct Target;
