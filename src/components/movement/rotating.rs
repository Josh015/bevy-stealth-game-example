use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

pub(super) struct RotatingPlugin;

impl Plugin for RotatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotating);
    }
}

/// Rotates an entity.
#[derive(Clone, Component, Debug, new)]
pub struct Rotating {
    start_rotation: Quat,
    end_rotation: Quat,

    #[new(default)]
    progress: f32,
}

fn rotating(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Rotating, &mut Transform)>,
) {
    for (entity, mut rotating, mut transform) in &mut query {
        rotating.progress = (rotating.progress + time.delta_seconds()).min(1.0);

        transform.rotation = rotating
            .start_rotation
            .slerp(rotating.end_rotation, rotating.progress);

        if rotating.progress == 1.0 {
            commands.entity(entity).remove::<Rotating>();
        }
    }
}
