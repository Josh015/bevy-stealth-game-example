use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

use super::MovingSpeed;

pub(super) struct MovingPlugin;

impl Plugin for MovingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, moving);
    }
}

/// Moves a [`MovingSpeed`] entity.
#[derive(Clone, Component, Debug, new)]
pub struct Moving {
    start: Vec3,
    end: Vec3,

    #[new(default)]
    progress: f32,
}

fn moving(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &MovingSpeed, &mut Moving, &mut Transform)>,
) {
    for (entity, moving_speed, mut moving, mut transform) in &mut query {
        moving.progress =
            (moving.progress + moving_speed.0 * time.delta_seconds()).min(1.0);

        transform.translation = moving.start.lerp(moving.end, moving.progress);

        if moving.progress == 1.0 {
            commands.entity(entity).remove::<Moving>();
        }
    }
}
