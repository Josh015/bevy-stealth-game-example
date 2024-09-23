use crate::{actions::*, components::*, system_sets::*};

use bevy::{color::palettes, pbr::NotShadowCaster, prelude::*};
use bevy_sequential_actions::*;
use rand::prelude::*;
use seldom_state::prelude::*;
use std::time::Duration;
use vleue_navigator::NavMesh;

pub(super) struct GuardPlugin;

impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                stun_response,
                chase_player,
                alarm_response,
                investigate_noise,
                search_for_player,
                patrol,
                guard_location,
            )
                .in_set(StopWhenPausedSet),
        );
        // .add_systems(
        //     Update,
        //     (give_target_auto, move_object)
        //         .run_if(in_state(GameState::Gameplay)),
        // );
    }
}

/// Required components for a [`Guard`] entity.
#[derive(Bundle)]
pub struct GuardBundle {
    pub guard: Guard,
    pub actions_bundle: ActionsBundle,
    pub state_machine: StateMachine,
    pub idle: GuardLocationState,
}

impl Default for GuardBundle {
    fn default() -> Self {
        // TODO: Check for special components in the transition checks for those
        // states.
        Self {
            guard: Guard,
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default()
                // Patrol
                // Search for player
                // Investigate noise
                // Stun response
                // Alarm response
                .trans::<GuardLocationState, _>(done(None), ChasePlayerState)
                .trans::<ChasePlayerState, _>(done(None), GuardLocationState),
            idle: GuardLocationState,
        }
    }
}

/// Designates a guard entity.
#[derive(Clone, Component, Debug, Default)]
pub struct Guard;

/// A [`Guard`] that can see the player.
#[derive(Clone, Component, Debug, Default)]
pub struct Vision {
    pub distance: f32,
    pub fov: f32,
}

// TODO: Use parent observer to bubble event up from child component.

/// A [`Guard`] that can hear and respond to sound waves.
#[derive(Clone, Component, Debug, Default)]
pub struct Hearing {
    pub radius: f32,
}

// TODO: Use parent observer to bubble event up from child component.

/// A [`Guard`] that's able to be stunned.
#[derive(Clone, Component, Debug, Default)]
pub struct Stunnable;

const IDLE_DELAY_MILLIS: u64 = 1_000;
const SPIN_DELAY_MILLIS: u64 = 400;

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct StunResponseState;

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct ChasePlayerState;

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct AlarmResponseState;

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct InvestigateNoiseState;

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct SearchForPlayerState;

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct PatrolState;

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct GuardLocationState;

fn stun_response(
    mut commands: Commands,
    query: Query<Entity, (With<Guard>, Added<StunResponseState>)>,
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

fn chase_player(
    mut commands: Commands,
    query: Query<(Entity, &Transform), (With<Guard>, Added<ChasePlayerState>)>,
    targets: Query<Entity, With<Target>>,
    navmeshes: Res<Assets<NavMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, transform) in &query {
        // Turn to face direction of player.
        // Parallel Actions:
        //   Play "Surprised" sound (blocking, once).
        //   Play "Surprised" animation (blocking, once).
        //   Emit "!" emote (blocking).
        // Emit emphasized "!" emote (non-blocking).
        // <path to player.>

        let Some(navmesh) = navmeshes.get(&Handle::default()) else {
            continue;
        };
        let mut x = 0.0;
        let mut z = 0.0;
        for _ in 0..50 {
            x = rand::thread_rng().gen_range(-50.0..50.0);
            z = rand::thread_rng().gen_range(-25.0..25.0);

            if navmesh.transformed_is_in_mesh(Vec3::new(x, 0.0, z)) {
                break;
            }
        }

        let Some(path) = navmesh
            .transformed_path(transform.translation, Vec3::new(x, 0.0, z))
        else {
            commands.entity(entity).insert(Done::Failure);
            continue;
        };

        let mut movement_path = path.path;

        movement_path.insert(0, transform.translation);

        for target in &targets {
            commands.entity(target).despawn_recursive();
        }

        commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Sphere { radius: 0.5 })),
                    material: materials.add(StandardMaterial {
                        base_color: palettes::css::RED.into(),
                        emissive: (palettes::css::RED * 5.0).into(),
                        ..default()
                    }),
                    transform: Transform::from_xyz(x, 0.0, z),
                    ..Default::default()
                },
                NotShadowCaster,
                Target,
                Path(movement_path.clone()),
            ))
            .with_children(|target| {
                target.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: palettes::css::RED.into(),
                        shadows_enabled: true,
                        range: 10.0,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 1.5, 0.0),
                    ..default()
                });
            });

        let mut agent_commands = commands.actions(entity);

        for point in movement_path {
            agent_commands.add(MoveAction::new(MoveTo::Destination(point)));
        }

        agent_commands.add_many(actions![
            |agent: Entity, world: &mut World| -> bool {
                world.entity_mut(agent).remove::<Path>();
                true
            },
            StateDoneAction::new(Done::Success)
        ]);
    }
}

fn alarm_response(
    mut commands: Commands,
    query: Query<Entity, (With<Guard>, Added<AlarmResponseState>)>,
) {
    for entity in &query {
        // <almost identical to Chase Player, minus sound and instead going to spot where camera saw player.>
    }
}

fn investigate_noise(
    mut commands: Commands,
    query: Query<Entity, (With<Guard>, Added<InvestigateNoiseState>)>,
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

fn search_for_player(
    mut commands: Commands,
    query: Query<Entity, (With<Guard>, Added<SearchForPlayerState>)>,
) {
    for entity in &query {
        // Turn to random direction.
        // Wait.
        // Turn to random direction.
        // Wait.
        // Parallel Actions:
        //   Emit "Frustrated" emote (blocking).
        //   Play "Frustrated" animation (blocking, once).
        // Done.
    }
}

fn patrol(
    mut commands: Commands,
    query: Query<Entity, (With<Guard>, Added<PatrolState>)>,
) {
    for entity in &query {
        // Repeat Sequence (forever):
        //   <generate for all patrol points>:
        //     Move to next point.
        //     Turn to face next point.
        //     Wait.
    }
}

fn guard_location(
    mut commands: Commands,
    query: Query<Entity, (With<Guard>, Added<GuardLocationState>)>,
) {
    for entity in &query {
        // <generate path back to guard location>
        //   Move to next point.
        // Turn to face guard direction.
        // Start "idle" animation (blocking, repeating).

        commands.actions(entity).add_many(actions![
            StartAnimationAction::new("idle".to_owned()),
            WaitAction::new(Duration::from_millis(IDLE_DELAY_MILLIS)),
            StateDoneAction::new(Done::Success)
        ]);
    }
}

// TODO: Remove these once done debugging.
#[derive(Component)]
pub struct Path(pub Vec<Vec3>);

#[derive(Component)]
pub struct Target;
