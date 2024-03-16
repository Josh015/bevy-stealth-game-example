use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

pub(super) struct MovingPlugin;

impl Plugin for MovingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, moving);
    }
}

/// Moves an entity.
#[derive(Clone, Component, Debug, new)]
pub struct Moving {
    start_position: Vec3,
    end_position: Vec3,

    #[new(default)]
    progress: f32,
}

fn moving(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Moving, &mut Transform)>,
) {
    for (entity, mut moving, mut transform) in &mut query {
        moving.progress = (moving.progress + time.delta_seconds()).min(1.0);

        transform.translation = moving
            .start_position
            .lerp(moving.end_position, moving.progress);

        if moving.progress == 1.0 {
            commands.entity(entity).remove::<Moving>();
        }
    }
}
