use crate::{system_params::*, system_sets::*, util::*};
use bevy::prelude::*;

const MOVING_ANIMATION: &str = "moving";
const DESTINATION_MARGIN_OF_ERROR: f32 = 0.001;

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // NOTE: Systems will malfunction if order isn't enforced!
        app.add_systems(
            Update,
            (movement_setup, movement_cleanup, translating, rotating)
                .chain()
                .in_set(StopWhenPausedSet),
        );
    }
}

/// Required components by an entity with [`Movement`].
#[derive(Bundle, Default)]
pub struct MovementBundle {
    pub linear_speed: LinearSpeed,
    pub angular_speed: AngularSpeed,
}

/// Linear speed in `meters/second`.
#[derive(Clone, Component, Debug)]
pub struct LinearSpeed(pub f32);

impl Default for LinearSpeed {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Angular speed in `radians/second`.
#[derive(Clone, Component, Debug)]
pub struct AngularSpeed(pub f32);

impl Default for AngularSpeed {
    fn default() -> Self {
        Self(std::f32::consts::TAU)
    }
}

/// Makes an entity transform in a specified way.
#[allow(dead_code)]
#[derive(Clone, Component, Debug)]
#[component(storage = "SparseSet")]
pub enum MoveTo {
    /// A point this entity is trying to reach.
    Destination(Vec3),

    /// A direction this entity wants to face.
    FaceDirection(Direction3d),

    /// An angle in radians that this entity wants to face.
    Heading(f32),
}

#[derive(Clone, Component, Debug)]
struct Movement {
    stored_animation: Option<Handle<AnimationClip>>,
}

#[derive(Clone, Component, Debug)]
struct Translating {
    destination: Vec3,
}

#[derive(Clone, Component, Debug)]
struct Rotating {
    yaw: f32,
    heading: f32,
}

fn movement_setup(
    mut commands: Commands,
    mut animations: Animations,
    query: Query<
        (Entity, &MoveTo, &Transform, Option<&Movement>),
        Changed<MoveTo>,
    >,
) {
    for (entity, move_to, transform, moving) in &query {
        let mut entity_commands = commands.entity(entity);
        let heading = match move_to {
            MoveTo::Destination(destination) => {
                let diff = *destination - transform.translation;

                // Translation.
                entity_commands.insert(Translating {
                    destination: *destination,
                });
                diff.x.atan2(diff.z)
            },
            MoveTo::FaceDirection(direction) => direction.x.atan2(direction.z),
            MoveTo::Heading(heading) => wrap_angle(*heading),
        };

        // Rotation.
        entity_commands.insert(Rotating {
            heading,
            yaw: transform.rotation.to_euler(EulerRot::YXZ).0,
        });

        // Start moving.
        if moving.is_none() {
            entity_commands.insert(Movement {
                // Save the currently playing animation for later.
                stored_animation: if let Some(current_animation) =
                    animations.get_current_clip(entity)
                {
                    Some(current_animation)
                } else {
                    None
                },
            });

            animations.play_clip(entity, MOVING_ANIMATION);
        }
    }
}

fn movement_cleanup(
    mut commands: Commands,
    mut animations: Animations,
    mut query: Query<
        (Entity, &Movement),
        (With<MoveTo>, Without<Translating>, Without<Rotating>),
    >,
) {
    for (entity, moving) in &mut query {
        // Clean up when everything is complete.
        commands.entity(entity).remove::<(Movement, MoveTo)>();

        // Restore the saved animation.
        if let Some(stored_animation) = &moving.stored_animation {
            animations.play_clip_handle(entity, stored_animation.clone_weak());
        }
    }
}

fn translating(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &Translating, &mut Transform, &LinearSpeed),
        With<MoveTo>,
    >,
) {
    for (entity, translating, mut transform, linear_speed) in &mut query {
        let diff = translating.destination - transform.translation;
        let dir = diff.normalize_or_zero();
        let distance_squared = diff.length_squared();
        let finished = distance_squared <= DESTINATION_MARGIN_OF_ERROR;

        transform.translation = if finished {
            commands.entity(entity).remove::<Translating>();
            translating.destination
        } else {
            transform.translation + dir * linear_speed.0 * time.delta_seconds()
        };
    }
}

fn rotating(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Rotating, &mut Transform, &AngularSpeed),
        With<MoveTo>,
    >,
) {
    for (entity, mut rotating, mut transform, angular_speed) in &mut query {
        let diff = wrap_angle(rotating.heading - rotating.yaw);
        let dir = diff.signum();
        let delta = dir * angular_speed.0 * time.delta_seconds();
        let finished = diff.abs() < delta.abs();

        rotating.yaw = if finished {
            commands.entity(entity).remove::<Rotating>();
            rotating.heading
        } else {
            wrap_angle(rotating.yaw + delta)
        };

        transform.rotation = Quat::from_rotation_y(rotating.yaw).normalize();
    }
}
