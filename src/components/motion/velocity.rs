use bevy::{ecs::prelude::*, prelude::*};

pub(super) struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (angular_velocity, velocity).chain());
    }
}

/// Rotational velocity which updates rotation over time.
#[derive(Clone, Component, Debug)]
pub struct AngularVelocity {
    pub axis: Direction3d,
    pub velocity: f32,
}

/// Velocity that updates translation over time.
#[derive(Clone, Component, Debug)]
pub struct Velocity(pub Vec3);

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

fn velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}
