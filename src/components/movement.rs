use bevy::prelude::*;

use crate::{AngularVelocity, CurrentAnimation, LinearVelocity};

const ANGULAR_VELOCITY_MARGIN_OF_ERROR: f32 = 0.0001;

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

fn destination_setup(
    mut commands: Commands,
    mut query: Query<
        (Entity, &Transform, &Destination, &LinearSpeed),
        Added<Destination>,
    >,
) {
    for (entity, transform, destination, linear_speed) in &mut query {
        let heading = (destination.0 - transform.translation).normalize();

        // NOTE: It's SUPER important to remove old animation FIRST!
        commands
            .entity(entity)
            .remove::<CurrentAnimation>()
            .insert((
                Heading(Direction3d::new_unchecked(heading)),
                LinearVelocity(heading * linear_speed.0),
                CurrentAnimation("moving".to_owned()),
            ));
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
            commands.entity(entity).remove::<(
                Destination,
                LinearVelocity,
                Heading,
                AngularVelocity,
                CurrentAnimation,
            )>();
        }
    }
}

fn destination_cleanup(
    mut commands: Commands,
    mut removed: RemovedComponents<Destination>,
) {
    for entity in removed.read() {
        commands
            .entity(entity)
            .remove::<(LinearVelocity, Heading, CurrentAnimation)>();
    }
}

fn heading_setup(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Heading, &AngularSpeed), Added<Heading>>,
) {
    for (entity, transform, heading, angular_speed) in &query {
        // NOTE: It's SUPER important to remove old animation FIRST!
        commands
            .entity(entity)
            .remove::<CurrentAnimation>()
            .insert((
                AngularVelocity {
                    axis: Direction3d::new_unchecked(
                        (-*transform.forward()).cross(*heading.0).normalize(),
                    ),
                    velocity: angular_speed.0,
                },
                CurrentAnimation("moving".to_owned()),
            ));
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
            let mut entity = commands.entity(entity);

            if has_destination {
                entity.remove::<AngularVelocity>();
            } else {
                entity.remove::<(Heading, AngularVelocity, CurrentAnimation)>();
            }
        }
    }
}

fn heading_cleanup(
    mut commands: Commands,
    mut removed: RemovedComponents<Heading>,
    query: Query<Has<Destination>>,
) {
    for entity in removed.read() {
        commands.entity(entity).remove::<AngularVelocity>();

        if let Ok(has_destination) = query.get(entity) {
            if !has_destination {
                commands.entity(entity).remove::<CurrentAnimation>();
            }
        }
    }
}
