use bevy::{ecs::prelude::*, prelude::*};

pub(super) struct RotatingPlugin;

impl Plugin for RotatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotating);
    }
}

/// Rotates an entity over time.
#[derive(Clone, Component, Debug)]
pub struct Rotating {
    pub axis: Direction3d,
    pub angle: f32,
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
