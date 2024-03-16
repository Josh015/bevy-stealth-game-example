use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

use super::TurningSpeed;

pub(super) struct TurningPlugin;

impl Plugin for TurningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, turning);
    }
}

/// Rotates a [`TurningSpeed`] entity.
#[derive(Clone, Component, Debug, new)]
pub struct Turning {
    start_rotation: Quat,
    end_rotation: Quat,

    #[new(default)]
    progress: f32,
}

fn turning(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &TurningSpeed, &mut Turning, &mut Transform)>,
) {
    for (entity, turning_speed, mut turning, mut transform) in &mut query {
        turning.progress = (turning.progress
            + turning_speed.0 * time.delta_seconds())
        .min(1.0);

        transform.rotation = turning
            .start_rotation
            .slerp(turning.end_rotation, turning.progress);

        if turning.progress == 1.0 {
            commands.entity(entity).remove::<Turning>();
        }
    }
}
