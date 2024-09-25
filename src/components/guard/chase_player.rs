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
                let mut x = 0.0;
                let mut z = 0.0;
                for _ in 0..50 {
                    x = rand::thread_rng().gen_range(-50.0..50.0);
                    z = rand::thread_rng().gen_range(-25.0..25.0);

                    if navmesh.transformed_is_in_mesh(Vec3::new(x, 0.0, z)) {
                        break;
                    }
                }

                let Some(path) = navmesh.transformed_path(
                    transform.translation,
                    Vec3::new(x, 0.0, z),
                ) else {
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
                            mesh: meshes
                                .add(Mesh::from(Sphere { radius: 0.5 })),
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
                    agent_commands
                        .add(MoveAction::new(MoveTo::Destination(point)));
                }

                agent_commands.add(
                    |agent: Entity, world: &mut World| -> bool {
                        world
                            .entity_mut(agent)
                            .remove::<Path>()
                            .insert(Escaped);
                        true
                    },
                );
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
                        MoveAction::new(MoveTo::FaceDirection(
                            random_direction
                        )),
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

// TODO: Remove these once done debugging.
#[derive(Component)]
pub struct Path(pub Vec<Vec3>);

#[derive(Component)]
pub struct Target;
