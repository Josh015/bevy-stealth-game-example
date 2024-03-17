use super::MovingSpeed;
use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

pub(super) struct MovingToPlugin;

impl Plugin for MovingToPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, moving_to);
    }
}

/// Moves a [`MovingSpeed`] entity to a new position before removing itself.
#[derive(Clone, Component, Debug, new)]
pub struct MovingTo {
    position: Vec3,
}

fn moving_to(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &MovingSpeed, &MovingTo, &mut Transform)>,
) {
    for (entity, moving_speed, moving_to, mut transform) in &mut query {
        let current_translation = transform.translation;

        transform.translation += (moving_to.position - current_translation)
            .normalize_or_zero()
            * moving_speed.0
            * time.delta_seconds();

        if transform.translation.distance(moving_to.position) <= 0.001 {
            commands.entity(entity).remove::<MovingTo>();
        }
    }
}
