use crate::common::{FORWARD_DIRECTION, MOVEMENT_TOLERANCE};

use super::{rotating::Rotating, TurningSpeed};
use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

pub(super) struct TurningPlugin;

impl Plugin for TurningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (start_turning_to, turning_to, clean_up_turning_to).chain(),
        );
    }
}

/// Rotates a [`TurningSpeed`] entity to a new rotation before removing itself.
#[derive(Clone, Component, Debug, new)]
pub struct TurningTo {
    direction: Direction3d,

    #[new(default)]
    is_finished: bool,
}

fn start_turning_to(
    mut commands: Commands,
    query: Query<
        (Entity, &TurningSpeed, &TurningTo, &Transform),
        Added<TurningTo>,
    >,
) {
    for (entity, turning_speed, turning_to, transform) in &query {
        commands.entity(entity).insert(Rotating::new(
            Direction3d::new_unchecked(
                (*transform.forward())
                    .cross(*turning_to.direction)
                    .normalize(),
            ),
            turning_speed.0,
        ));
    }
}

fn turning_to(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TurningTo, &mut Transform)>,
) {
    for (entity, mut turning_to, mut transform) in &mut query {
        // Delay removal by one update to prevent visual snapping to final rotation.
        if turning_to.is_finished {
            commands.entity(entity).remove::<TurningTo>();
            transform.rotation = Quat::from_rotation_arc(
                FORWARD_DIRECTION,
                *turning_to.direction,
            );
        } else {
            turning_to.is_finished =
                (*transform.forward()).dot(*turning_to.direction).abs()
                    >= 1.0 - MOVEMENT_TOLERANCE;
        }
    }
}

fn clean_up_turning_to(
    mut commands: Commands,
    mut removed: RemovedComponents<TurningTo>,
    query: Query<Entity, With<Rotating>>,
) {
    // Clean up associated components if this one is removed early.
    for entity in removed.read() {
        if query.contains(entity) {
            commands.entity(entity).remove::<Rotating>();
        }
    }
}
