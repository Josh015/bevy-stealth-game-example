use crate::common::constants::MOVEMENT_TOLERANCE;

use super::{translating::Translating, turning_to::TurningTo, MovingSpeed};
use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

pub(super) struct MovingToPlugin;

impl Plugin for MovingToPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (start_moving_to, moving_to, clean_up_moving_to).chain(),
        );
    }
}

/// Moves a [`MovingSpeed`] entity to a new position before removing itself.
#[derive(Clone, Component, Debug, new)]
pub struct MovingTo {
    position: Vec3,

    #[new(default)]
    is_finished: bool,
}

fn start_moving_to(
    mut commands: Commands,
    query: Query<
        (Entity, &MovingSpeed, &MovingTo, &Transform),
        Added<MovingTo>,
    >,
) {
    for (entity, moving_speed, moving_to, transform) in &query {
        let moving_direction =
            (moving_to.position - transform.translation).normalize();

        commands.entity(entity).insert((
            Translating::new(moving_direction * moving_speed.0),
            TurningTo::new(Direction3d::new_unchecked(moving_direction)),
        ));
    }
}

fn moving_to(
    mut commands: Commands,
    mut query: Query<(Entity, &mut MovingTo, &mut Transform)>,
) {
    for (entity, mut moving_to, mut transform) in &mut query {
        // Delay removal by one update to prevent visual snapping to final position.
        if moving_to.is_finished {
            commands.entity(entity).remove::<MovingTo>();
            transform.translation = moving_to.position;
        } else {
            moving_to.is_finished =
                moving_to.position.distance(transform.translation)
                    <= MOVEMENT_TOLERANCE;
        }
    }
}

fn clean_up_moving_to(
    mut commands: Commands,
    mut removed: RemovedComponents<MovingTo>,
    query: Query<Entity, Or<(With<Translating>, With<TurningTo>)>>,
) {
    // Clean up associated components if this one is removed early.
    for entity in removed.read() {
        if query.contains(entity) {
            commands.entity(entity).remove::<(Translating, TurningTo)>();
        }
    }
}
