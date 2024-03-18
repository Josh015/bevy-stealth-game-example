use bevy::{ecs::prelude::*, prelude::*};

pub(super) struct AngularVelocityPlugin;

impl Plugin for AngularVelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, angular_velocity);
    }
}

/// Rotational velocity which updates rotation over time.
#[derive(Clone, Component, Debug)]
pub struct AngularVelocity {
    pub axis: Direction3d,
    pub velocity: f32,
}

fn angular_velocity(
    time: Res<Time>,
    mut query: Query<(&AngularVelocity, &mut Transform)>,
) {
    for (angular_velocity, mut transform) in &mut query {
        transform.rotation = (transform.rotation
            * Quat::from_axis_angle(
                *angular_velocity.axis,
                angular_velocity.velocity * time.delta_seconds(),
            ))
        .normalize();
    }
}
