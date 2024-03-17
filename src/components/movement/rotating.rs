use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

pub(super) struct RotatingPlugin;

impl Plugin for RotatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotating);
    }
}

/// Rotates an entity over time.
#[derive(Clone, Component, Debug, new)]
pub struct Rotating {
    axis: Direction3d,
    angle: f32,
}

fn rotating(time: Res<Time>, mut query: Query<(&Rotating, &mut Transform)>) {
    for (rotating, mut transform) in &mut query {
        transform.rotation = (transform.rotation
            * Quat::from_axis_angle(
                *rotating.axis,
                rotating.angle * time.delta_seconds(),
            ))
        .normalize();
    }
}
