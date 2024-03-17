use super::TurningSpeed;
use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

pub(super) struct TurningPlugin;

impl Plugin for TurningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (start_turning_to, turning_to).chain());
    }
}

/// Rotates a [`TurningSpeed`] entity.
#[derive(Clone, Component, Debug, new)]
pub struct TurningTo {
    end: Quat,

    #[new(default)]
    start: Quat,

    #[new(default)]
    progress: f32,
}

fn start_turning_to(
    mut query: Query<(&mut TurningTo, &Transform), Added<TurningTo>>,
) {
    for (mut turning, transform) in &mut query {
        turning.start = transform.rotation;
    }
}

fn turning_to(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &TurningSpeed, &mut TurningTo, &mut Transform)>,
) {
    for (entity, turning_speed, mut turning_to, mut transform) in &mut query {
        turning_to.progress = (turning_to.progress
            + turning_speed.0 * time.delta_seconds())
        .min(1.0);

        transform.rotation =
            turning_to.start.slerp(turning_to.end, turning_to.progress);

        if turning_to.progress == 1.0 {
            commands.entity(entity).remove::<TurningTo>();
        }
    }
}
