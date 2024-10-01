use std::time::Duration;

use bevy::{
    ecs::query::QueryFilter, prelude::*, time::common_conditions::on_timer,
};
use vleue_navigator::NavMesh;

use crate::prelude::*;

const MOVING_ANIMATION: &str = "moving";
const DESTINATION_MARGIN_OF_ERROR: f32 = 0.001;

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                store_current_animation,
                restart_stored_animation,
                start_turning,
                turn_toward_heading,
                refresh_route::<Changed<Destination>>,
                refresh_route::<With<Destination>>
                    .run_if(on_timer(Duration::from_secs_f32(0.1))),
                follow_route_to_destination,
            )
                .in_set(StoppedWhenPausedSet),
        );
    }
}

/// A point that the entity will navigate to reach.
#[derive(Clone, Component, Debug, Default)]
pub struct Destination(pub Vec3);

/// A direction that the entity will rotate to face.
#[derive(Clone, Component, Debug)]
pub struct Heading(pub f32);

impl Heading {
    pub fn from_vector(direction: Vec3) -> Self {
        Self(direction.x.atan2(direction.z))
    }
}

/// Saved animation clip that can be restored later.
#[derive(Clone, Component, Debug)]
pub struct StoredAnimation(pub AnimationNodeIndex);

/// Rotation around the Y-axis required to reach a [Heading].
#[derive(Clone, Component, Debug)]
pub struct Yaw(pub f32);

/// The navigation route an entity will take to reach a [Destination].
#[derive(Clone, Component, Debug)]
pub struct Route {
    pub next: Vec3,
    pub remaining: Vec<Vec3>,
}

fn store_current_animation(
    mut commands: Commands,
    mut animations: Animations,
    query: Query<
        Entity,
        (
            Without<StoredAnimation>,
            Or<(Added<Destination>, Added<Heading>)>,
        ),
    >,
) {
    for entity in &query {
        if let Some(current_animation) =
            animations.get_current_animation(entity)
        {
            commands
                .entity(entity)
                .insert(StoredAnimation(current_animation));
        }

        animations.play_clip_name(entity, MOVING_ANIMATION);
    }
}

fn restart_stored_animation(
    mut commands: Commands,
    mut animations: Animations,
    query: Query<
        (Entity, &StoredAnimation),
        (Without<Destination>, Without<Heading>),
    >,
) {
    for (entity, stored_animation) in &query {
        animations.play_clip(entity, stored_animation.0);

        commands.entity(entity).remove::<StoredAnimation>();
    }
}

fn start_turning(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Changed<Heading>>,
) {
    for (entity, transform) in &query {
        commands
            .entity(entity)
            .insert(Yaw(transform.rotation.to_euler(EulerRot::YXZ).0));
    }
}

fn turn_toward_heading(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Yaw, &Heading, &AngularSpeed),
        (Without<Destination>, With<Heading>),
    >,
) {
    for (entity, mut transform, mut yaw, heading, angular_speed) in &mut query {
        // Rotate to face next point on path.
        let diff = wrap_angle(heading.0 - yaw.0);
        let dir = diff.signum();
        let delta = dir * angular_speed.0 * time.delta_seconds();
        let rotation_finished = diff.abs() < delta.abs();

        yaw.0 = if rotation_finished {
            heading.0
        } else {
            wrap_angle(yaw.0 + delta)
        };

        transform.rotation = Quat::from_rotation_y(yaw.0).normalize();

        if !rotation_finished {
            continue;
        }

        commands.entity(entity).remove::<(Heading, Yaw)>();
    }
}

fn refresh_route<QF: QueryFilter>(
    mut commands: Commands,
    navmeshes: Res<Assets<NavMesh>>,
    query: Query<(Entity, &Destination, &Transform), QF>,
) {
    for (entity, destination, transform) in &query {
        let Some(navmesh) = navmeshes.get(&Handle::default()) else {
            continue;
        };
        let Some(path) =
            navmesh.transformed_path(transform.translation, destination.0)
        else {
            commands
                .entity(entity)
                .remove::<(Destination, Route, Yaw, Heading)>();
            continue;
        };

        let mut remaining = path.path;
        remaining.reverse();

        let Some(next) = remaining.pop() else {
            continue;
        };

        let diff = next - transform.translation;

        commands
            .entity(entity)
            .insert((Heading::from_vector(diff), Route { next, remaining }));
    }
}

fn follow_route_to_destination(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Route,
        &mut Transform,
        &mut Yaw,
        &mut Heading,
        &LinearSpeed,
        &AngularSpeed,
    )>,
) {
    for (
        entity,
        mut path_to,
        mut transform,
        mut yaw,
        mut heading,
        linear_speed,
        angular_speed,
    ) in &mut query
    {
        // Translate toward next point on path.
        let diff = path_to.next - transform.translation;
        let dir = diff.normalize_or_zero();
        let distance_squared = diff.length_squared();
        let translation_finished =
            distance_squared <= DESTINATION_MARGIN_OF_ERROR;

        transform.translation = if translation_finished {
            path_to.next
        } else {
            transform.translation + dir * linear_speed.0 * time.delta_seconds()
        };

        // Rotate to face next point on path.
        let diff = wrap_angle(heading.0 - yaw.0);
        let dir = diff.signum();
        let delta = dir * angular_speed.0 * time.delta_seconds();
        let rotation_finished = diff.abs() < delta.abs();

        yaw.0 = if rotation_finished {
            heading.0
        } else {
            wrap_angle(yaw.0 + delta)
        };

        transform.rotation = Quat::from_rotation_y(yaw.0).normalize();

        // Can't wait for rotation because that can cause abrupt stops.
        if !translation_finished {
            continue;
        }

        // Queue up next point along path.
        if let Some(next) = path_to.remaining.pop() {
            let diff = next - transform.translation;
            *heading = Heading::from_vector(diff);
            path_to.next = next;
        } else {
            commands
                .entity(entity)
                .remove::<(Route, Destination, Heading, Yaw)>();
        }
    }
}
