use std::time::Duration;

use bevy::{color::palettes, pbr::NotShadowCaster, prelude::*};
use bevy_sequential_actions::*;
use rand::prelude::*;
use seldom_state::prelude::*;
use vleue_navigator::NavMesh;

use crate::prelude::*;

pub(super) struct ChasePlayerPlugin;

impl Plugin for ChasePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, chase_player.in_set(StoppedWhenPausedSet));
    }
}

const ESCAPED_FACE_DIRECTION_DELAY: Duration = Duration::from_millis(1500);

#[derive(Clone, Component, Copy, Default, Reflect)]
#[component(storage = "SparseSet")]
pub enum ChasePlayer {
    #[default]
    Surprised,
    Chasing,
    Escaped,
}

fn chase_player(
    mut commands: Commands,
    query: Query<
        (Entity, &Transform, &ChasePlayer),
        (With<Guard>, Changed<ChasePlayer>),
    >,
    targets: Query<Entity, With<Target>>,
    navmeshes: Res<Assets<NavMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use ChasePlayer::*;

    for (entity, transform, chase_player) in &query {
        match chase_player {
            Surprised => {
                let mut agent_commands = commands.actions(entity);

                // Turn to face direction of player.
                // Parallel Actions:
                //   Play "Surprised" sound (blocking, once).
                //   Play "Surprised" animation (blocking, once).
                //   Emit "!" emote (blocking).
                // Emit emphasized "!" emote (non-blocking).

                agent_commands.add(
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(Chasing);
                        true
                    },
                );
            },
            Chasing => {
                // TODO: Isolate this code behind SawPlayerEvent!
                let Some(navmesh) = navmeshes.get(&Handle::default()) else {
                    continue;
                };
                let mut destination = Vec3::ZERO;
                let mut rng = SmallRng::from_entropy();
                let mut valid_point = false;

                for _ in 0..50 {
                    destination.x = rng.gen_range(-50.0..50.0);
                    destination.z = rng.gen_range(-25.0..25.0);
                    valid_point = navmesh.transformed_is_in_mesh(destination);

                    if valid_point {
                        break;
                    }
                }

                if !valid_point {
                    continue;
                }

                for target in &targets {
                    commands.entity(target).despawn_recursive();
                }

                commands
                    .spawn((
                        PbrBundle {
                            mesh: meshes
                                .add(Mesh::from(Sphere { radius: 0.5 })),
                            material: materials.add(StandardMaterial {
                                base_color: palettes::css::RED.into(),
                                emissive: (palettes::css::RED * 5.0).into(),
                                ..default()
                            }),
                            transform: Transform::from_translation(destination),
                            ..Default::default()
                        },
                        NotShadowCaster,
                        Target,
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

                commands.actions(entity).add_many(actions![
                    MoveToAction::new(destination),
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(Escaped);
                        true
                    },
                ]);
            },
            Escaped => {
                let mut agent_commands = commands.actions(entity);
                let mut rng = SmallRng::from_entropy();

                for _ in 0..2 {
                    let mut random_vector = Vec3::ZERO;
                    random_vector.x = rng.gen_range(-1.0..=1.0);
                    random_vector.z = rng.gen_range(-1.0..=1.0);

                    let random_direction =
                        Dir3::new_unchecked(random_vector.normalize());

                    agent_commands.add_many(actions![
                        FaceDirectionAction::new(random_direction),
                        WaitAction::new(ESCAPED_FACE_DIRECTION_DELAY),
                    ]);
                }

                agent_commands.add_many(actions![
                    ParallelActions::new(actions![
                        AnimationAction::new("frustrated".into()),
                        EmoteAction::new("frustrated".into())
                    ]),
                    |agent: Entity, world: &mut World| -> bool {
                        world.entity_mut(agent).insert(Done::Failure);
                        true
                    },
                ]);
            },
        }
    }
}

#[derive(Component)]
pub struct Target;
