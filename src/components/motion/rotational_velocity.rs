use bevy::{ecs::prelude::*, prelude::*};

pub(super) struct RotationalVelocityPlugin;

impl Plugin for RotationalVelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotational_velocity);
    }
}

/// Rotational velocity which updates rotation over time.
#[derive(Clone, Component, Debug)]
pub struct RotationalVelocity {
    pub axis: Direction3d,
    pub velocity: f32,
}

fn rotational_velocity(
    time: Res<Time>,
    mut query: Query<(&RotationalVelocity, &mut Transform)>,
) {
    for (rotational_velocity, mut transform) in &mut query {
        transform.rotation = (transform.rotation
            * Quat::from_axis_angle(
                *rotational_velocity.axis,
                rotational_velocity.velocity * time.delta_seconds(),
            ))
        .normalize();
    }
}
