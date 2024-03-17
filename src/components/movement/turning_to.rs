use crate::common::constants::MOVEMENT_TOLERANCE;

use super::TurningSpeed;
use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

pub(super) struct TurningPlugin;

impl Plugin for TurningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (start_turning_to, turning_to).chain());
    }
}

/// Rotates a [`TurningSpeed`] entity to a new rotation before removing itself.
#[derive(Clone, Component, Debug, new)]
pub struct TurningTo {
    direction: Direction3d,

    #[new(default)]
    axis: Vec3,
}

fn start_turning_to(
    mut query: Query<(&mut TurningTo, &Transform), Added<TurningTo>>,
) {
    for (mut turning_to, transform) in &mut query {
        turning_to.axis = (*transform.forward()).cross(*turning_to.direction);
    }
}

fn turning_to(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &TurningSpeed, &TurningTo, &mut Transform)>,
) {
    for (entity, turning_speed, turning_to, mut transform) in &mut query {
        if (*transform.forward()).dot(*turning_to.direction).abs()
            < 1.0 - MOVEMENT_TOLERANCE
        {
            transform.rotation = (transform.rotation
                * Quat::from_axis_angle(
                    turning_to.axis,
                    turning_speed.0 * time.delta_seconds(),
                ))
            .normalize();
        } else {
            commands.entity(entity).remove::<TurningTo>();
        }
    }
}
