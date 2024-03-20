use bevy::prelude::*;

pub(super) struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (linear_velocity, angular_velocity).chain());
    }
}

/// Linear velocity that updates translation over time.
#[derive(Clone, Component, Debug)]
pub struct LinearVelocity(pub Vec3);

/// Angular velocity that updates rotation over time.
#[derive(Clone, Component, Debug)]
pub struct AngularVelocity {
    pub axis: Direction3d,
    pub velocity: f32,
}

fn linear_velocity(
    time: Res<Time>,
    mut query: Query<(&LinearVelocity, &mut Transform)>,
) {
    for (linear_velocity, mut transform) in &mut query {
        transform.translation += linear_velocity.0 * time.delta_seconds();
    }
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
