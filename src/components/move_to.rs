use super::speed::*;
use crate::{game_state::*, system_params::*, util::*};
use bevy::prelude::*;

const MOVING_ANIMATION: &str = "moving";
const DESTINATION_MARGIN_OF_ERROR: f32 = 0.001;

pub(super) struct MoveToPlugin;

impl Plugin for MoveToPlugin {
    fn build(&self, app: &mut App) {
        // NOTE: Systems will malfunction if order isn't enforced!
        app.add_systems(
            Update,
            (
                move_to_started,
                move_to_completed,
                move_to_removed,
                translating,
                rotating,
            )
                .chain()
                .in_set(StoppedWhenPausedSet),
        );
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
    FaceDirection(Dir3),

    /// An angle in radians that this entity wants to face.
    Heading(f32),
}

#[derive(Clone, Component, Debug)]
struct StoredAnimation(Handle<AnimationClip>);

#[derive(Clone, Component, Debug)]
struct Translating {
    destination: Vec3,
}

#[derive(Clone, Component, Debug)]
struct Rotating {
    yaw: f32,
    heading: f32,
}

fn move_to_started(
    mut commands: Commands,
    mut animations: Animations,
    query: Query<
        (Entity, &MoveTo, &Transform, Has<StoredAnimation>),
        Changed<MoveTo>,
    >,
) {
    for (entity, move_to, transform, has_stored_animation) in &query {
        // Queue up automatic translation and rotation.
        let mut entity_commands = commands.entity(entity);
        let heading = match move_to {
            MoveTo::Heading(heading) => wrap_angle(*heading),
            MoveTo::FaceDirection(direction) => direction.x.atan2(direction.z),
            MoveTo::Destination(destination) => {
                let diff = *destination - transform.translation;

                entity_commands.insert(Translating {
                    destination: *destination,
                });
                diff.x.atan2(diff.z)
            },
        };

        entity_commands.insert(Rotating {
            heading,
            yaw: transform.rotation.to_euler(EulerRot::YXZ).0,
        });

        // Save currently playing animation to restore later.
        if !has_stored_animation {
            // TODO: Implement the new AnimationGraph stuff!
            // if let Some(current_animation) = animations.get_current_clip(entity)
            // {
            //     entity_commands.insert(StoredAnimation(current_animation));
            // }

            animations.play_clip(entity, MOVING_ANIMATION);
        }
    }
}

fn move_to_completed(
    mut commands: Commands,
    query: Query<
        Entity,
        (With<MoveTo>, Without<Translating>, Without<Rotating>),
    >,
) {
    for entity in &query {
        commands.entity(entity).remove::<MoveTo>();
    }
}

fn move_to_removed(
    mut commands: Commands,
    mut animations: Animations,
    mut removed: RemovedComponents<MoveTo>,
    query: Query<&StoredAnimation>,
) {
    for entity in removed.read() {
        // Clean up all related components just in case.
        commands
            .entity(entity)
            .remove::<(Translating, Rotating, StoredAnimation)>();

        // Restore the saved animation.
        if let Ok(stored_animation) = query.get(entity) {
            animations
                .play_clip_handle(entity, stored_animation.0.clone_weak());
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
