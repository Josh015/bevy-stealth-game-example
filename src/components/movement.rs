use bevy::prelude::*;

use crate::{AngularVelocity, LinearVelocity};

use super::Animator;

const ANGULAR_VELOCITY_MARGIN_OF_ERROR: f32 = 0.0001;
const MOVING_ANIMATION: &str = "moving";

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // The order is important for correct rotations, so don't mess with it!
        app.add_systems(
            Update,
            (
                destination_setup,
                destination_check_progress,
                destination_cleanup,
                heading_setup,
                heading_check_progress,
                heading_cleanup,
            )
                .chain(),
        );
    }
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

/// A point this entity is trying to reach.
#[derive(Clone, Component, Debug)]
pub struct Destination(pub Vec3);

/// A direction this entity wants to face.
#[derive(Clone, Component, Debug)]
pub struct Heading(pub Direction3d);

/// Stores currently running animation for later restoration.
#[derive(Clone, Component, Debug, Default)]
pub struct StoredAnimation(pub Handle<AnimationClip>);

fn destination_setup(
    mut commands: Commands,
    mut animator: Animator,
    mut query: Query<
        (Entity, &Transform, &Destination, &LinearSpeed),
        Added<Destination>,
    >,
) {
    for (entity, transform, destination, linear_speed) in &mut query {
        let heading = (destination.0 - transform.translation).normalize();
        let mut entity_commands = commands.entity(entity);

        entity_commands.insert((
            Heading(Direction3d::new_unchecked(heading)),
            LinearVelocity(heading * linear_speed.0),
        ));

        if let Some(current_animation) = animator.get_current_animation(entity)
        {
            entity_commands.insert(StoredAnimation(current_animation));
        }

        animator.play_animation_name(entity, MOVING_ANIMATION);
    }
}

fn destination_check_progress(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Destination, &Heading)>,
) {
    for (entity, mut transform, destination, heading) in &mut query {
        if (destination.0 - transform.translation)
            .normalize()
            .dot(*heading.0)
            <= 0.0
        {
            transform.translation = destination.0;
            commands
                .entity(entity)
                .remove::<(Destination, LinearVelocity)>();
        }
    }
}

fn destination_cleanup(
    mut commands: Commands,
    mut animator: Animator,
    mut removed: RemovedComponents<Destination>,
    query: Query<&StoredAnimation>,
) {
    for entity in removed.read() {
        commands.entity(entity).remove::<LinearVelocity>();

        if let Ok(stored_animation) = query.get(entity) {
            animator
                .play_animation_handle(entity, stored_animation.0.clone_weak())
        }
    }
}

fn heading_setup(
    mut commands: Commands,
    mut animator: Animator,
    query: Query<
        (
            Entity,
            &Transform,
            &Heading,
            &AngularSpeed,
            Has<Destination>,
        ),
        Added<Heading>,
    >,
) {
    for (entity, transform, heading, angular_speed, has_destination) in &query {
        let mut entity_commands = commands.entity(entity);

        entity_commands.insert((AngularVelocity {
            axis: Direction3d::new_unchecked(
                (-*transform.forward()).cross(*heading.0).normalize(),
            ),
            velocity: angular_speed.0,
        },));

        if !has_destination {
            if let Some(current_animation) =
                animator.get_current_animation(entity)
            {
                entity_commands.insert(StoredAnimation(current_animation));
            }
        }

        animator.play_animation_name(entity, MOVING_ANIMATION);
    }
}

fn heading_check_progress(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Heading, Has<Destination>)>,
) {
    for (entity, transform, heading, has_destination) in &query {
        // Negate forward() because glTF models typically face +Z axis.
        if (-*transform.forward()).dot(*heading.0).abs()
            >= 1.0 - ANGULAR_VELOCITY_MARGIN_OF_ERROR
        {
            let mut entity_commands = commands.entity(entity);

            if has_destination {
                entity_commands.remove::<AngularVelocity>();
            } else {
                entity_commands.remove::<(Heading, AngularVelocity)>();
            }
        }
    }
}

fn heading_cleanup(
    mut commands: Commands,
    mut animator: Animator,
    mut removed: RemovedComponents<Heading>,
    query: Query<Option<&StoredAnimation>, Without<Destination>>,
) {
    for entity in removed.read() {
        commands.entity(entity).remove::<AngularVelocity>();

        if let Ok(Some(stored_animation)) = query.get(entity) {
            animator
                .play_animation_handle(entity, stored_animation.0.clone_weak());
        }
    }
}
